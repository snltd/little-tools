mod action;
mod new_name;

use camino::Utf8PathBuf;
use clap::{CommandFactory, Parser, Subcommand};
use regex::Regex;
use std::process;

use crate::replace::{FromPattern, RenameOpts, Replacing};
use action::ActionOpts;

#[derive(Parser)]
#[command(about = "Batch renamer")]
struct Cli {
    /// print the rename operations without doing them
    #[arg(short, long)]
    noop: bool,
    /// with --noop, only print target names
    #[arg(short, long)]
    terse: bool,
    /// overwrite any existing files
    #[arg(short, long)]
    clobber: bool,
    /// print every operation
    #[arg(short, long)]
    verbose: bool,
    /// show fully qualified pathnames in verbose output
    #[arg(short, long)]
    full: bool,
    /// exclude the filename extension from operations
    #[arg(short, long)]
    exclude_ext: bool,
    /// print arguments for `git mv`
    #[clap(short = 'G', long = "git", conflicts_with = "noop")]
    git: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(alias = "r")]
    Replace {
        /// treat FROM as a literal string rather than a regex pattern
        #[arg(short = 'l', long)]
        literal: bool,
        /// replace all occurrences of FROM
        #[arg(short = 'a', long = "all", conflicts_with = "replace_nth")]
        replace_all: bool,
        /// replace nth occurrence of FROM. Zero-indexed, can be specified multiple times
        #[arg(
            short = 'm',
            long = "match",
            conflicts_with = "replace_all",
            value_parser
        )]
        nth: Vec<usize>,
        /// pattern to replace, which can be a Rust regex unless --literal is specified
        from: String,
        /// string with which to replace FROM
        to: String,
        #[arg(required = true)]
        files: Vec<Utf8PathBuf>,
    },
    /// prefix filename(s) instead of replacing
    Prefix {
        text: String,
        #[arg(required = true)]
        files: Vec<Utf8PathBuf>,
    },
    /// suffix filename(s) instead of replacing
    Suffix {
        text: String,
        #[arg(required = true)]
        files: Vec<Utf8PathBuf>,
    },
    /// change the extension
    Ext {
        ext: String,
        #[arg(required = true)]
        files: Vec<Utf8PathBuf>,
    },
    /// change the value of the Nth number in the name, instead of replacing
    Number {
        #[arg(short, long)]
        up: i64,
        #[arg(short, long)]
        down: i64,
        #[arg(required = true)]
        files: Vec<Utf8PathBuf>,
    },
}

struct GlobalOpts {
    noop: bool,
    terse: bool,
    verbose: bool,
    clobber: bool,
    full: bool,
    exclude_ext: bool,
    git: bool,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let globals = GlobalOpts {
        noop: cli.noop,
        terse: cli.terse,
        verbose: cli.verbose,
        clobber: cli.clobber,
        full: cli.full,
        exclude_ext: cli.exclude_ext,
        git: cli.git,
    };

    let mut ret = 0;

    let (to, files): (Option<String>, Vec<Utf8PathBuf>) = match cli.command {
        Commands::Replace {
            literal,
            replace_all,
            nth,
            from,
            to,
            files,
        } => {}
        Commands::Prefix { text, files } => (None, cli.rest.clone()),
        Commands::Suffix { text, files } => {}
        Commands::Ext { ext, files } => {}
        Commands::Number { up, down, files } => {}
    };

    if cli.prefix || cli.suffix {
    } else {
        let mut rest = cli.rest.clone();
        if rest.is_empty() {
            Cli::command()
                .error(
                    clap::error::ErrorKind::MissingRequiredArgument,
                    "the following required arguments were not provided:\n  <TO>\n  <FILES>...",
                )
                .exit();
        }
        let to = rest.remove(0).into_string();
        (Some(to), rest)
    };

    if files.is_empty() {
        Cli::command()
            .error(
                clap::error::ErrorKind::MissingRequiredArgument,
                "the following required arguments were not provided:\n  <FILES>...",
            )
            .exit();
    }

    let replacing = if cli.suffix {
        Replacing::Suffix(cli.pattern.clone())
    } else if cli.prefix {
        Replacing::Prefix(cli.pattern.clone())
    } else if cli.replace_all {
        Replacing::All
    } else if cli.replace_nth.is_empty() {
        Replacing::Indices(vec![0])
    } else if let Some(rn) = cli.bump_number {
        Replacing::Renumber(rn)
    } else {
        Replacing::Indices(cli.replace_nth)
    };

    let pattern = if cli.literal {
        FromPattern::Literal(cli.pattern.clone())
    } else {
        match Regex::new(&cli.pattern) {
            Ok(rx) => FromPattern::Regex(rx),
            Err(e) => {
                eprintln!("ERROR compiling regex {}: {e:#}", cli.pattern);
                process::exit(2);
            }
        }
    };

    let rename_opts = RenameOpts {
        from: pattern,
        to,
        replacing,
    };

    let action_list = match action::action_list(files, cli.exclude_ext, cli.extension, &rename_opts)
    {
        Ok(list) => list,
        Err(e) => {
            eprintln!("ERROR: {e:#}");
            process::exit(3);
        }
    };

    // If we don't think we can rename everything, we'll rename nothing
    if let Err(e) = action::check_action_list(&action_list) {
        eprintln!("ERROR: {e:#}");
        process::exit(4);
    }

    let action_opts = ActionOpts {
        clobber: cli.clobber,
        full_names: cli.full_names,
        git: cli.git,
        noop: cli.noop,
        terse_output: cli.terse_output,
        verbose: cli.verbose,
    };

    for (src, target) in &action_list {
        if let Err(e) = action::rename_file(src, target, &action_opts) {
            ret = 1;
            eprintln!("ERROR renaming {src} -> {target}: {e:#}");
        }
    }

    process::exit(ret)
}

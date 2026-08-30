mod action;
mod replace;

use camino::Utf8PathBuf;
use clap::Parser;
use regex::Regex;
use std::process;

use crate::replace::{FromPattern, RenameOpts, Replacing};
use action::ActionOpts;

#[derive(Parser, Debug)]
#[clap(version, about = "Batch renamer", long_about = None)]
struct Cli {
    /// treat PATTERN as a literal string rather than a regex pattern
    #[clap(short, long)]
    literal: bool,
    /// replace all occurrences of PATTERN
    #[clap(short = 'a', long = "all", conflicts_with = "replace_nth")]
    replace_all: bool,
    /// print the rename operations without doing them
    #[clap(short, long)]
    noop: bool,
    /// overwrite any existing files
    #[clap(short, long)]
    clobber: bool,
    /// include the filename extension in replacements
    #[clap(short, long)]
    include_ext: bool,
    /// set extension to given arg
    #[clap(short = 'E', long)]
    extension: Option<String>,
    /// show fully qualified pathnames in verbose output
    #[clap(short, long = "full")]
    full_names: bool,
    /// only replace the nth match (starts at 0)
    #[clap(
        short = 'm',
        long = "match",
        conflicts_with = "replace_all",
        value_parser
    )]
    replace_nth: Vec<usize>,
    /// with -n, only print target names
    #[clap(short, long = "terse")]
    terse_output: bool,
    /// be verbose
    #[clap(short, long)]
    verbose: bool,
    /// print arguments for git mv
    #[clap(short = 'G', long = "git", conflicts_with = "noop")]
    git: bool,
    /// pattern to replace. Supports Rust regexes
    #[clap(value_parser)]
    from: String,
    /// string that should replace <pattern>. Supports Rust capture groups, like ${1}
    #[clap(value_parser)]
    to: String,
    /// files to rename
    #[arg(required = true)]
    files: Vec<Utf8PathBuf>,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let mut ret = 0;

    let replacing = if cli.replace_all {
        Replacing::All
    } else if cli.replace_nth.is_empty() {
        Replacing::Indices(vec![0])
    } else {
        Replacing::Indices(cli.replace_nth)
    };

    let from = if cli.literal {
        FromPattern::Literal(cli.from.clone())
    } else {
        match Regex::new(&cli.from) {
            Ok(rx) => FromPattern::Regex(rx),
            Err(e) => {
                eprintln!("ERROR compiling regex {}: {e:#}", cli.from);
                process::exit(2);
            }
        }
    };

    let rename_opts = RenameOpts {
        from,
        to: cli.to,
        replacing,
    };

    let action_list =
        match action::action_list(cli.files, cli.include_ext, cli.extension, &rename_opts) {
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

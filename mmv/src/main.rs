mod action_list;
mod mv;
mod new_name;
mod types;

use crate::action_list::ActionList;
use crate::types::{
    GlobalOpts, RenameAction, RenumberOpts, ReplaceLiteralOpts, ReplaceRegexOpts, Replacements,
};
use camino::Utf8PathBuf;
use clap::{Parser, Subcommand};
use regex::Regex;
use std::process;

#[derive(Parser)]
#[command(version, about = "Batch renamer.")]
struct Cli {
    /// Print the rename operations without doing them
    #[arg(short, long, global = true)]
    noop: bool,
    /// With --noop, only print target names
    #[arg(short, long, global = true)]
    terse: bool,
    /// Overwrite any existing files
    #[arg(short, long, global = true)]
    clobber: bool,
    /// Print every operation
    #[arg(short, long, global = true)]
    verbose: bool,
    /// Show fully qualified pathnames in verbose output
    #[arg(short, long, global = true)]
    full: bool,
    /// Include the filename extension from operations. Normally mmv operates only on the stem
    /// part of the filename
    #[arg(short = 'e', long, global = true)]
    include_ext: bool,
    /// Print arguments for `git mv`
    #[clap(short = 'G', long = "git", global = true, conflicts_with = "noop")]
    git: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Rename multiple files with find/replace.
    #[command(alias = "r")]
    Replace {
        /// Replace nth occurrence of FROM. Zero-indexed, and can be specified multiple times
        #[arg(
            short = 'N',
            long = "replace-nth",
            value_parser,
            conflicts_with = "replace_all"
        )]
        index: Vec<usize>,
        /// Replace all occurrences of FROM. If neither --replace-all nor --replace-nth are
        /// supplied, the first match will be replaced
        #[arg(short = 'a', long)]
        replace_all: bool,
        /// Treat FROM as a literal string rather than a regex pattern
        #[arg(short, long)]
        literal: bool,
        /// Pattern to replace, which can be a Rust regex unless --literal is specified
        from: String,
        /// String with which to replace FROM. If FROM is a regex with capture groups, TO can
        /// use backreferences
        to: String,
        /// One or more files
        #[arg(required = true)]
        files: Vec<Utf8PathBuf>,
    },
    /// Prefix filename(s) with a given string.
    Prefix {
        /// Prefix to use. It is joined directly to the start of the filename, with no separator
        prefix: String,
        /// One or more files
        #[arg(required = true)]
        files: Vec<Utf8PathBuf>,
    },
    /// Suffix filename(s) with a given string
    Suffix {
        /// Suffix to use. Unless --include-ext is used, it will be added BEFORE the file
        /// extension. The join is direct, with no separator.
        suffix: String,
        /// One or more files
        #[arg(required = true)]
        files: Vec<Utf8PathBuf>,
    },
    /// Change the extension of one or more files
    Extension {
        /// New extension. If the file has no extension, one will be added
        extension: String,
        /// One or more files
        #[arg(required = true)]
        files: Vec<Utf8PathBuf>,
    },
    /// Change the value of the nth number in one or more filename(s)
    Renumber {
        /// Which number to change. Numbers are any distinct, greedy, \d+ match
        #[arg(short, long, required = true)]
        index: Option<usize>,
        /// Increment the number by this amount
        #[arg(short, long, conflicts_with = "down")]
        up: Option<i64>,
        /// Decrement the number by this amount
        #[arg(short, long)]
        down: Option<i64>,
        /// Pad altered numbers with leading zeros. Value is the number of chars to pad to
        #[arg(short, long)]
        zeros: Option<u8>,
        /// One or more files
        #[arg(required = true)]
        files: Vec<Utf8PathBuf>,
    },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let global_opts = GlobalOpts {
        noop: cli.noop,
        terse: cli.terse,
        verbose: cli.verbose,
        clobber: cli.clobber,
        full: cli.full,
        include_ext: cli.include_ext,
        git: cli.git,
    };

    let (rename_action, files) = match cli.command {
        Commands::Replace {
            literal,
            replace_all,
            index,
            from,
            to,
            files,
        } => {
            let replacements = if replace_all {
                Replacements::All
            } else if index.is_empty() {
                Replacements::Indices(vec![0])
            } else {
                Replacements::Indices(index)
            };

            if literal {
                (
                    RenameAction::ReplaceLiteral(ReplaceLiteralOpts {
                        replacements,
                        from,
                        to,
                    }),
                    files,
                )
            } else {
                let from = match Regex::new(&from) {
                    Ok(rx) => rx,
                    Err(e) => {
                        eprintln!("ERROR compiling regex {}: {e:#}", from);
                        process::exit(2);
                    }
                };

                (
                    RenameAction::ReplaceRegex(ReplaceRegexOpts {
                        replacements,
                        from,
                        to,
                    }),
                    files,
                )
            }
        }
        Commands::Prefix {
            prefix: text,
            files,
        } => (RenameAction::Prefix(text), files),
        Commands::Suffix {
            suffix: text,
            files,
        } => (RenameAction::Suffix(text), files),
        Commands::Extension { extension, files } => (RenameAction::Extension(extension), files),
        Commands::Renumber {
            index,
            up,
            down,
            files,
            zeros,
        } => {
            let by: i64 = if let Some(val) = up {
                val
            } else if let Some(val) = down {
                -val
            } else {
                panic!("NOOOOO!");
            };

            let index = index.unwrap_or(1);

            (
                RenameAction::Renumber(RenumberOpts { by, index, zeros }),
                files,
            )
        }
    };

    let action_list = match ActionList::new(files, &rename_action, &global_opts) {
        Ok(list) => list,
        Err(e) => {
            eprintln!("ERROR: {e:#}");
            process::exit(3);
        }
    };

    // If we don't think we can rename everything, we'll rename nothing
    if let Err(e) = action_list.check() {
        eprintln!("ERROR: {e:#}");
        process::exit(4);
    }

    // Again, first fail -- exit
    if let Err(e) = action_list.rename(&global_opts) {
        eprintln!("ERROR: {e:#}");
        process::exit(1);
    }

    process::exit(0)
}

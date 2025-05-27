use anyhow::{anyhow, Context};
use camino::Utf8PathBuf;
use clap::Parser;
use common::verbose;
use std::fs;
mod replace;

#[derive(Parser, Debug)]
#[clap(version, about = "Batch renamer", long_about = None)]
struct Cli {
    /// pattern to replace. Supports Rust regexes
    #[clap(value_parser)]
    pattern: String,
    /// string that should replace <pattern>. Supports Rust capture groups, like ${1}
    #[clap(value_parser)]
    replace: String,
    /// replace all occurrences of pattern
    #[clap(short = 'a', long = "all")]
    replace_all: bool,
    /// just print the rename operations
    #[clap(short, long)]
    noop: bool,
    /// overwrite existing files
    #[clap(short, long)]
    clobber: bool,
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
    replace_nth: Option<usize>,
    /// with -n, only print target names
    #[clap(short, long = "terse")]
    terse_output: bool,
    /// be verbose
    #[clap(short, long)]
    verbose: bool,
    /// print arguments for git mv
    #[clap(short = 'G', long = "git", conflicts_with = "noop")]
    git: bool,
    /// files to rename
    #[arg(required = true)]
    files: Vec<Utf8PathBuf>,
}

struct Opts {
    pattern: String,
    replace_nth: Option<usize>,
    replace: String,
    replace_all: bool,
    noop: bool,
    clobber: bool,
    full_names: bool,
    terse_output: bool,
    verbose: bool,
    git: bool,
}

fn main() {
    let cli = Cli::parse();
    let mut ret = 0;

    let opts = Opts {
        pattern: cli.pattern,
        replace: cli.replace,
        replace_all: cli.replace_all,
        replace_nth: cli.replace_nth,
        noop: cli.noop,
        clobber: cli.clobber,
        full_names: cli.full_names,
        terse_output: cli.terse_output,
        verbose: cli.verbose,
        git: cli.git,
    };

    for file in &cli.files {
        if let Err(e) = process_file(file, &opts) {
            ret = 1;
            eprintln!("ERROR: {}: {}", file, e);
        }
    }

    std::process::exit(ret)
}

fn process_file(source: &Utf8PathBuf, opts: &Opts) -> anyhow::Result<bool> {
    let target = target_path(source, opts)?;
    let source_name;
    let target_name;

    if opts.full_names {
        source_name = source.to_string();
        target_name = target.to_string();
    } else {
        source_name = source
            .file_name()
            .context("cannot get source file name")?
            .to_owned();
        target_name = target
            .file_name()
            .context("cannot get target file name")?
            .to_owned();
    };

    if target_name == *source_name {
        verbose!(opts, "{}: no change", source_name);
        return Ok(false);
    }

    if opts.git {
        println!("git mv {} {}", source, target);
        Ok(true)
    } else {
        if opts.terse_output {
            println!("{}", target_name);
        } else {
            verbose!(opts, "{} -> {}", source_name, target_name);
        }

        if opts.noop {
            return Ok(false);
        }

        rename(source, &target, opts)?;
        Ok(true)
    }
}

fn target_path(source: &Utf8PathBuf, opts: &Opts) -> anyhow::Result<Utf8PathBuf> {
    let source = source.canonicalize_utf8()?;

    let dir = source.parent().context("cannot get parent")?;
    let name = source.file_name().context("cannot get file name")?;
    let pattern = opts.pattern.as_str();
    let replace = opts.replace.as_str();

    let target_name = match opts.replace_nth {
        Some(index) => replace::nth(pattern, replace, name, index),
        None => {
            if opts.replace_all {
                replace::all(pattern, replace, name)
            } else {
                replace::first(pattern, replace, name)
            }
        }
    };

    let target = dir.join(target_name);
    Ok(target)
}

fn rename(src: &Utf8PathBuf, dest: &Utf8PathBuf, opts: &Opts) -> anyhow::Result<()> {
    if dest.exists() && !opts.clobber {
        return Err(anyhow!("filename collision"));
    }

    Ok(fs::rename(src, dest)?)
}

#[cfg(test)]
mod test {
    use super::*;
    use test_utils::fixture;

    #[test]
    fn target_requires_no_change() {
        let opts = Opts {
            pattern: String::from("does_not_match"),
            replace: String::from("new"),
            clobber: false,
            full_names: false,
            git: false,
            noop: true,
            verbose: true,
            replace_all: false,
            replace_nth: None,
            terse_output: false,
        };

        assert_eq!(
            fixture("file_file_file.txt"),
            target_path(&fixture("file_file_file.txt"), &opts).unwrap()
        );
    }

    #[test]
    fn target_requires_change_of_first_match() {
        let opts = Opts {
            pattern: String::from("file"),
            replace: String::from("new"),
            clobber: false,
            full_names: false,
            git: false,
            noop: true,
            verbose: true,
            replace_all: false,
            replace_nth: None,
            terse_output: false,
        };

        assert_eq!(
            fixture("new_file_file.txt"),
            target_path(&fixture("file_file_file.txt"), &opts).unwrap()
        );
    }

    #[test]
    fn target_requires_change_of_second_match() {
        let opts = Opts {
            pattern: String::from("file"),
            replace: String::from("new"),
            clobber: false,
            full_names: false,
            git: false,
            noop: true,
            verbose: true,
            replace_all: false,
            replace_nth: Some(1),
            terse_output: false,
        };

        assert_eq!(
            fixture("file_new_file.txt"),
            target_path(&fixture("file_file_file.txt"), &opts).unwrap()
        );
    }
    #[test]
    fn target_requires_change_of_all_matches() {
        let opts = Opts {
            pattern: String::from("file"),
            replace: String::from("new"),
            clobber: false,
            full_names: false,
            git: false,
            noop: true,
            verbose: true,
            replace_all: true,
            replace_nth: None,
            terse_output: false,
        };

        assert_eq!(
            fixture("new_new_new.txt"),
            target_path(&fixture("file_file_file.txt"), &opts).unwrap()
        );
    }

    #[test]
    fn target_requires_change_of_all_regex() {
        let opts = Opts {
            pattern: String::from("f([a-z]+)e"),
            replace: String::from("b${1}l"),
            replace_all: true,
            clobber: false,
            full_names: false,
            git: false,
            noop: true,
            verbose: true,
            replace_nth: None,
            terse_output: false,
        };

        assert_eq!(
            fixture("bill_bill_bill.txt"),
            target_path(&fixture("file_file_file.txt"), &opts).unwrap()
        );
    }
}

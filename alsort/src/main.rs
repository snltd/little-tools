use anyhow::Context;
use camino::{Utf8Path, Utf8PathBuf};
use clap::Parser;
use common::verbose;
use std::{fs, process};

#[derive(Parser)]
#[clap(version,
    about = "Sort files into directories based on their first letter",
    long_about = None)]
struct Cli {
    /// Root target directory
    #[clap(short = 'R', long = "root", default_value = ".")]
    root: Utf8PathBuf,
    /// Say what would happen without actually doing it
    #[clap(short, long)]
    noop: bool,
    /// Be verbose
    #[clap(short, long)]
    verbose: bool,
    /// Group into abc, def, ... rather than a, b, d, e, f, ...
    #[clap(short, long)]
    group: bool,
    /// Files to process
    #[clap(required = true)]
    files: Vec<Utf8PathBuf>,
}

struct Opts {
    root: Utf8PathBuf,
    noop: bool,
    verbose: bool,
    group: bool,
}

fn main() {
    let cli = Cli::parse();
    let mut exit_code = 0;

    let opts = Opts {
        root: cli.root,
        noop: cli.noop,
        verbose: cli.verbose,
        group: cli.group,
    };

    for path in &cli.files {
        if let Err(e) = process(path, &opts) {
            eprintln!("ERROR: failed to process {path}: {e}");
            exit_code = 1;
        }
    }

    process::exit(exit_code);
}

fn process(path: &Utf8Path, opts: &Opts) -> anyhow::Result<bool> {
    let f = path.canonicalize_utf8()?;
    let basename = f.file_name().context("failed to get basename")?;
    let raw_initial = basename.to_lowercase().chars().next();
    let initial = raw_initial.context("failed to get initial")?;
    let target_dir = target_from_initial(initial, &opts.root, opts.group);

    if !target_dir.exists() {
        verbose!(opts, "creating target {target_dir}");

        if !opts.noop {
            fs::create_dir_all(&target_dir)?;
        }
    }

    if target_dir == f {
        Ok(false)
    } else {
        verbose!(opts, "{f} -> {target_dir}");

        if !opts.noop {
            let target_file = target_dir.join(basename);
            fs::rename(&f, &target_file)?;
        }

        Ok(true)
    }
}

fn target_from_initial(initial: char, root: &Utf8PathBuf, group: bool) -> Utf8PathBuf {
    if group {
        root.join(group_from_initial(initial))
    } else {
        root.join(initial.to_string())
    }
}

fn group_from_initial(initial: char) -> String {
    match initial {
        '0'..='9' => "0-9",
        'a'..='c' => "abc",
        'd'..='f' => "def",
        'g'..='i' => "ghi",
        'j'..='l' => "jkl",
        'm'..='o' => "mno",
        'p'..='s' => "pqrs",
        't'..='v' => "tuv",
        'w'..='z' => "wxyz",
        _ => "symbols",
    }
    .to_string()
}

#[cfg(test)]
mod test {
    use super::*;
    use camino_tempfile_ext::prelude::*;

    #[test]
    fn test_process() {
        let temp = Utf8TempDir::new().unwrap();
        temp.child("a_file.txt").touch().unwrap();
        temp.child("abc").create_dir_all().unwrap();
        let file_under_test = temp.path().join("a_file.txt");

        assert!(file_under_test.exists());

        assert!(
            process(
                &file_under_test,
                &Opts {
                    root: temp.path().to_path_buf(),
                    verbose: false,
                    group: true,
                    noop: false,
                },
            )
            .unwrap()
        );

        assert!(!file_under_test.exists());
        assert!(temp.path().join("abc").join("a_file.txt").exists());
    }

    #[test]
    fn test_process_ignores_target() {
        let temp = Utf8TempDir::new().unwrap();
        temp.child("a_file.txt").touch().unwrap();
        temp.child("abc").create_dir_all().unwrap();
        let file_under_test = temp.path().join("abc").canonicalize_utf8().unwrap();

        assert!(file_under_test.exists());

        assert!(
            !process(
                &file_under_test,
                &Opts {
                    root: temp.path().to_path_buf().canonicalize_utf8().unwrap(),
                    verbose: false,
                    group: true,
                    noop: false,
                },
            )
            .unwrap()
        );
    }

    #[test]
    fn test_target_from_initial() {
        assert_eq!(
            Utf8PathBuf::from("/test/dir/wxyz"),
            target_from_initial('x', &Utf8PathBuf::from("/test/dir"), true),
        );

        assert_eq!(
            Utf8PathBuf::from("/test/dir/x"),
            target_from_initial('x', &Utf8PathBuf::from("/test/dir"), false),
        );
    }

    #[test]
    fn test_group_from_initial() {
        assert_eq!("abc".to_string(), group_from_initial('b'));
        assert_eq!("tuv".to_string(), group_from_initial('t'));
        assert_eq!("wxyz".to_string(), group_from_initial('w'));
        assert_eq!("pqrs".to_string(), group_from_initial('p'));
        assert_eq!("pqrs".to_string(), group_from_initial('s'));
        assert_eq!("jkl".to_string(), group_from_initial('j'));
        assert_eq!("abc".to_string(), group_from_initial('b'));
        assert_eq!("def".to_string(), group_from_initial('e'));
        assert_eq!("0-9".to_string(), group_from_initial('7'));
        assert_eq!("symbols".to_string(), group_from_initial('!'));
        assert_eq!("symbols".to_string(), group_from_initial('_'));
        assert_eq!("symbols".to_string(), group_from_initial(' '));
        assert_eq!("symbols".to_string(), group_from_initial('*'));
    }
}

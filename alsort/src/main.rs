use anyhow::anyhow;
use camino::Utf8PathBuf;
use clap::Parser;
use std::fs;

#[derive(Parser)]
#[clap(version, about = "Sort files into directories based on their first letter", long_about = None)]
struct Cli {
    #[clap(short = 'R', long = "root", default_value = ".")]
    root: Utf8PathBuf,
    #[clap(short, long)]
    noop: bool,
    #[clap(short, long)]
    verbose: bool,
    #[clap(short, long)]
    group: bool,
    #[clap(required = true)]
    files: Vec<Utf8PathBuf>,
}

fn main() {
    let cli = Cli::parse();
    let mut errs = false;

    for f in &cli.files {
        if let Err(e) = process(f, &cli) {
            println!("ERROR: failed to process #{}: #{}", &f, e);
            errs = true;
        }
    }

    if errs {
        std::process::exit(1);
    }
}

fn process(file: &Utf8PathBuf, cli: &Cli) -> anyhow::Result<bool> {
    let f = file.canonicalize_utf8()?;
    let basename = match f.file_name() {
        Some(fname) => fname,
        None => return Err(anyhow!("failed to get basename")),
    };

    let raw_initial = basename.to_lowercase().chars().next();

    let initial = match raw_initial {
        Some(letter) => letter,
        None => return Err(anyhow!("failed to get initial")),
    };

    let target_dir = target_from_initial(initial, &cli.root, cli.group).canonicalize_utf8()?;

    if target_dir == f {
        return Ok(false);
    }

    if cli.verbose || cli.noop {
        println!("{} -> {}", f, target_dir);
    }

    if cli.noop {
        return Ok(true);
    }

    if !target_dir.exists() {
        fs::create_dir(&target_dir)?;
    }

    let target_file = target_dir.join(basename);

    fs::rename(&f, &target_file)?;
    Ok(true)
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
                &Cli {
                    root: temp.path().to_path_buf(),
                    verbose: false,
                    group: true,
                    files: vec![],
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
        let file_under_test = temp.path().join("abc");

        assert!(file_under_test.exists());

        assert!(
            !process(
                &file_under_test,
                &Cli {
                    root: temp.path().to_path_buf(),
                    verbose: false,
                    group: true,
                    files: vec![],
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

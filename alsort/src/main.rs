use clap::Parser;
use std::fs;
use std::io::{self, Error, ErrorKind};
use std::path::{Path, PathBuf};

#[derive(Parser)]
#[clap(version, about = "Sort files into directories based on their first letter", long_about = None)]
struct Cli {
    #[clap(short = 'R', long = "root", default_value = ".")]
    root: PathBuf,
    #[clap(short, long)]
    noop: bool,
    #[clap(short, long)]
    verbose: bool,
    #[clap(short, long)]
    group: bool,
    #[clap(required = true)]
    files: Vec<PathBuf>,
}

fn main() {
    let cli = Cli::parse();
    let mut errs = false;

    for f in &cli.files {
        if let Err(e) = process(f, &cli) {
            println!("ERROR: failed to process #{}: #{}", &f.display(), e);
            errs = true;
        }
    }

    if errs {
        std::process::exit(1);
    }
}

fn process(file: &Path, cli: &Cli) -> Result<(), io::Error> {
    let f = file.canonicalize()?;
    let basename = match f.file_name() {
        Some(fname) => fname,
        None => return Err(Error::new(ErrorKind::Other, "failed to get basename")),
    };

    let raw_initial = basename
        .to_string_lossy()
        .to_string()
        .to_lowercase()
        .chars()
        .next();

    let initial = match raw_initial {
        Some(letter) => letter,
        None => return Err(Error::new(ErrorKind::Other, "failed to get initial")),
    };

    let target_dir = match target_from_initial(initial, &cli.root, cli.group) {
        Ok(dir) => dir,
        Err(_) => return Err(Error::new(ErrorKind::Other, "failed to get target dir")),
    };

    if cli.verbose || cli.noop {
        println!("{} -> {}", f.display(), target_dir.display());
    }

    if cli.noop {
        return Ok(());
    }

    if !target_dir.exists() {
        fs::create_dir(&target_dir)?;
    }

    let target_file = target_dir.join(basename);

    fs::rename(&f, &target_file)
}

fn target_from_initial(initial: char, root: &Path, group: bool) -> Result<PathBuf, io::Error> {
    if group {
        Ok(root.join(group_from_initial(initial)))
    } else {
        Ok(root.join(initial.to_string()))
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

use clap::Parser;
use std::fs::ReadDir;
use std::path::{Path, PathBuf};

#[derive(Parser, Debug)]
#[clap(version, about = "Counts the files in the given directories", long_about = None)]
struct Args {
    /// directories
    #[clap(value_parser)]
    files: Vec<PathBuf>,

    /// recurse and count all files
    #[clap(short = 'r', long = "recurse")]
    recurse: bool,

    /// Only count files, omitting directories
    #[clap(short, long)]
    nodirs: bool,
}

fn main() {
    let args = Args::parse();

    if args.files.is_empty() {
        eprintln!("No files.");
        std::process::exit(1);
    }

    process(&args);
}

fn count_files_recurse(dir: &Path, nodirs: bool, mut count: usize) -> Option<usize> {
    if let Ok(d) = dir.read_dir() {
        for f in d.flatten() {
            let p = f.path();

            if p.is_dir() {
                if !nodirs {
                    count += 1;
                }

                if let Some(n) = count_files_recurse(&p, nodirs, count) {
                    count = n;
                }
            } else {
                count += 1;
            }
        }
    } else {
        return None;
    }

    Some(count)
}

fn count_files_only(dir: ReadDir) -> usize {
    dir.filter_map(|f| f.ok())
        .filter(|f| f.path().is_file())
        .count()
}

fn count_files(dir: &Path, recurse: bool, nodirs: bool) -> Option<usize> {
    if recurse {
        count_files_recurse(dir, nodirs, 0)
    } else {
        match dir.read_dir() {
            Ok(d) => {
                if nodirs {
                    Some(count_files_only(d))
                } else {
                    Some(d.count())
                }
            }
            Err(_) => None,
        }
    }
}

fn process(args: &Args) {
    for dir in args.files.iter().filter(|f| f.is_dir()) {
        if let Some(c) = count_files(dir, args.recurse, args.nodirs) {
            println!("\t{}\t{}", c, dir.display());
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_count_files_recurse() {
        assert_eq!(
            Some(0),
            count_files(&PathBuf::from("spec/resources/a"), true, false)
        );
        assert_eq!(
            Some(2),
            count_files(&PathBuf::from("spec/resources/b"), true, true)
        );
        assert_eq!(
            Some(3),
            count_files(&PathBuf::from("spec/resources/b"), true, false)
        );
        assert_eq!(
            Some(6),
            count_files(&PathBuf::from("spec/resources/c"), true, true)
        );
        assert_eq!(
            Some(8),
            count_files(&PathBuf::from("spec/resources/c"), true, false)
        );
    }

    #[test]
    fn test_count_files() {
        assert_eq!(
            Some(0),
            count_files(&PathBuf::from("spec/resources/a"), false, false)
        );
        assert_eq!(
            Some(2),
            count_files(&PathBuf::from("spec/resources/b"), false, true)
        );
        assert_eq!(
            Some(3),
            count_files(&PathBuf::from("spec/resources/b"), false, false)
        );
        assert_eq!(
            Some(2),
            count_files(&PathBuf::from("spec/resources/c"), false, true)
        );
        assert_eq!(
            Some(4),
            count_files(&PathBuf::from("spec/resources/c"), false, false)
        );
        assert_eq!(
            None,
            count_files(&PathBuf::from("spec/resources/z"), false, false)
        );
    }
}

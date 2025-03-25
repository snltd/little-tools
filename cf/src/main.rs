use camino::{ReadDirUtf8, Utf8Path, Utf8PathBuf};
use clap::Parser;
use std::process;

#[derive(Parser, Debug)]
#[clap(version, about = "Counts the files in the given directories", long_about = None)]
struct Cli {
    /// Recurse, and count all files
    #[clap(short = 'r', long = "recurse")]
    recurse: bool,
    /// Only count files, omitting directories
    #[clap(short, long)]
    nodirs: bool,
    /// Directories to assess
    #[arg(required = true)]
    files: Vec<Utf8PathBuf>,
}

fn main() {
    let cli = Cli::parse();
    let mut exit_code = 0;

    for dir in cli.files.iter() {
        if dir.is_dir() && dir.exists() {
            if let Some(c) = count_files(dir, cli.recurse, cli.nodirs) {
                println!("\t{}\t{}", c, dir);
            }
        } else {
            eprintln!("ERROR: {} is not a directory", dir);
            exit_code = 1;
        }
    }

    process::exit(exit_code);
}

fn count_files(dir: &Utf8Path, recurse: bool, nodirs: bool) -> Option<usize> {
    if recurse {
        count_files_recurse(dir, nodirs, 0)
    } else {
        match dir.read_dir_utf8() {
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

fn count_files_recurse(dir: &Utf8Path, nodirs: bool, mut count: usize) -> Option<usize> {
    if let Ok(d) = dir.read_dir_utf8() {
        for f in d.flatten() {
            let p = f.path();

            if p.is_dir() {
                if !nodirs {
                    count += 1;
                }

                if let Some(n) = count_files_recurse(p, nodirs, count) {
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

fn count_files_only(dir: ReadDirUtf8) -> usize {
    dir.filter_map(|f| f.ok())
        .filter(|f| f.path().is_file())
        .count()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_count_files_recurse() {
        assert_eq!(
            2,
            count_files(&Utf8PathBuf::from("tests/resources/a"), true, true).unwrap()
        );
        assert_eq!(
            6,
            count_files(&Utf8PathBuf::from("tests/resources/b"), true, true).unwrap()
        );
        assert_eq!(
            8,
            count_files(&Utf8PathBuf::from("tests/resources/b"), true, false).unwrap()
        );
    }

    #[test]
    fn test_count_files() {
        assert_eq!(
            2,
            count_files(&Utf8PathBuf::from("tests/resources/a"), false, true).unwrap()
        );
        assert_eq!(
            2,
            count_files(&Utf8PathBuf::from("tests/resources/b"), false, true).unwrap()
        );
        assert_eq!(
            4,
            count_files(&Utf8PathBuf::from("tests/resources/b"), false, false).unwrap()
        );
        assert_eq!(
            None,
            count_files(&Utf8PathBuf::from("tests/resources/z"), false, false)
        );
    }
}

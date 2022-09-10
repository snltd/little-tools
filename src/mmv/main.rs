use clap::Parser;
use std::fs;
use std::path::PathBuf;
mod replace;

#[derive(Parser, Debug)]
#[clap(version, about = "Batch renamer", long_about = None)]
struct Args {
    /// pattern to replace. Supports Rust regexes
    #[clap(value_parser)]
    pattern: String,

    /// string that should replace <pattern>. Supports Rust capture groups, like ${1}
    #[clap(value_parser)]
    replace: String,

    /// files to rename
    #[clap(value_parser)]
    files: Vec<PathBuf>,

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
        conflicts_with = "replace-all",
        value_parser
    )]
    replace_nth: Option<usize>,

    /// with -n, only print target names
    #[clap(short, long = "terse")]
    terse_output: bool,

    /// be verbose
    #[clap(short, long)]
    verbose: bool,
}

fn main() {
    let args = Args::parse();

    if args.files.is_empty() {
        eprintln!("No files.");
        std::process::exit(1);
    }

    process(&args);
}

fn process(args: &Args) {
    for file in &args.files {
        match process_file(file.to_path_buf(), args) {
            Ok(msg) => {
                if (args.verbose && !msg.ends_with("no change")) || args.noop {
                    println!("{} {}", file.display(), msg);
                }
            }
            Err(e) => println!("ERROR: {}: {}", file.display(), e),
        }
    }
}

fn process_file(file: PathBuf, args: &Args) -> Result<String, String> {
    let target = target_path(file.clone(), args)?;

    if target == file {
        return Ok(String::from(": no change"));
    }

    if !args.noop {
        rename(file.clone(), target.clone(), args.clobber)?;
    }

    process_info(file, target, args.verbose || args.noop, args.terse_output)
}

fn process_info(
    file: PathBuf,
    target: PathBuf,
    verbose: bool,
    terse: bool,
) -> Result<String, String> {
    if !verbose {
        return Ok(String::new());
    }

    let mut target_display = target.as_os_str();

    if terse {
        if let Some(p) = target.file_name() {
            target_display = p;
        };
    }

    let display_name = match target_display.to_str() {
        Some(display_str) => display_str,
        None => return Err(String::from("cannot parse filename")),
    };

    Ok(format!("{} -> {}", file.display(), display_name))
}

fn target_path(file: PathBuf, args: &Args) -> Result<PathBuf, String> {
    if !file.exists() {
        return Err(String::from("file not found"));
    }

    let filename = match file.to_str() {
        Some(f) => f,
        None => return Err(String::from("cannot parse filename")),
    };

    let target = match args.replace_nth {
        Some(index) => replace::nth(&args.pattern, &args.replace, filename, index),
        None => {
            if args.replace_all {
                replace::all(&args.pattern, &args.replace, filename)
            } else {
                replace::first(&args.pattern, &args.replace, filename)
            }
        }
    };

    Ok(PathBuf::from(&target))
}

#[allow(dead_code)]
fn rename(src: PathBuf, dest: PathBuf, clobber: bool) -> Result<(), String> {
    if dest.exists() && !clobber {
        return Err(String::from("filename collision"));
    }

    match fs::rename(src, dest) {
        Ok(_) => Ok(()),
        Err(_) => Err(String::from("failed to rename")),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_process_output() {
        assert_eq!(
            Ok(String::from("")),
            process_info(default_file(), default_file(), false, false)
        );

        assert_eq!(
            Ok(String::from("dir/file_a -> dir/file_b")),
            process_info(
                PathBuf::from("dir/file_a"),
                PathBuf::from("dir/file_b"),
                true,
                false
            )
        );

        assert_eq!(
            Ok(String::from("dir/file_a -> file_b")),
            process_info(
                PathBuf::from("dir/file_a"),
                PathBuf::from("dir/file_b"),
                true,
                true
            )
        );
    }

    #[test]
    fn target_does_not_exist() {
        assert_eq!(
            Err(String::from("file not found")),
            target_path(PathBuf::from("/file/does/not/exist"), &default_args())
        );
    }

    #[test]
    fn target_requires_no_change() {
        let mut args = default_args();
        args.pattern = String::from("nomatch");

        assert_eq!(Ok(default_file()), target_path(default_file(), &args));
    }

    #[test]
    fn target_requires_change_of_first_match() {
        assert_eq!(
            Ok(PathBuf::from("tests/data/mmv/new_file_file.txt")),
            target_path(default_file(), &default_args())
        );
    }

    #[test]
    fn target_requires_change_of_second_match() {
        let mut args = default_args();
        args.replace_nth = Some(1);

        assert_eq!(
            Ok(PathBuf::from("tests/data/mmv/file_new_file.txt")),
            target_path(default_file(), &args)
        );
    }
    #[test]
    fn target_requires_change_of_all_matches() {
        let mut args = default_args();
        args.replace_all = true;

        assert_eq!(
            Ok(PathBuf::from("tests/data/mmv/new_new_new.txt")),
            target_path(default_file(), &args)
        );
    }

    #[test]
    fn target_requires_change_of_all_regex() {
        let mut args = default_args();
        args.pattern = String::from("f([a-z]+)e");
        args.replace = String::from("b${1}l");
        args.replace_all = true;

        assert_eq!(
            Ok(PathBuf::from("tests/data/mmv/bill_bill_bill.txt")),
            target_path(default_file(), &args)
        );
    }

    fn default_file() -> PathBuf {
        PathBuf::from("tests/data/mmv/file_file_file.txt")
    }

    fn default_args() -> Args {
        Args {
            pattern: String::from("file"),
            replace: String::from("new"),
            clobber: false,
            full_names: false,
            noop: true,
            verbose: true,
            replace_all: false,
            replace_nth: None,
            terse_output: false,
            files: vec![default_file()],
        }
    }
}

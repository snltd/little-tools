use anyhow::anyhow;
use camino::{Utf8Path, Utf8PathBuf};
use clap::Parser;
use filetime::{set_file_times, FileTime};
use glob::glob;
use std::collections::BTreeMap;
use std::fs::{metadata, File};
use std::io;
use std::time::SystemTime;
use time::{format_description::well_known::Rfc2822, OffsetDateTime};

type MTimeMap = BTreeMap<Utf8PathBuf, SystemTime>;

struct Opts {
    pub verbose: bool,
    pub noop: bool,
}

#[derive(Parser)]
#[clap(version, about = "Recursively aligns file timestamps between two directories", long_about = None)]
struct Cli {
    /// Print what would happen, without doing it
    #[clap(short, long)]
    noop: bool,
    /// Be verbose
    #[clap(short, long)]
    verbose: bool,
    /// source directory name
    #[arg(required = true)]
    source: String,
    /// dest directory name
    #[arg(required = true)]
    dest: String,
}

fn touch_directory(source: &Utf8PathBuf, dest: &Utf8PathBuf, opts: &Opts) -> anyhow::Result<()> {
    if !source.exists() {
        return Err(anyhow!("No source directory: {}", source));
    }

    if !dest.exists() {
        return Err(anyhow!("No destination directory: {}", dest));
    }

    let source_timestamps = timestamps_for(source, opts);
    let dest_timestamps = timestamps_for(dest, opts);
    let mut errs = 0;

    for (file, ts) in source_timestamps {
        if let Some(dest_ts) = dest_timestamps.get(&file) {
            let target_file = dest.join(&file);
            if &ts != dest_ts {
                if opts.noop || opts.verbose {
                    println!("{} -> {}", target_file, format_time(ts));
                }

                if !opts.noop && set_timestamp(&target_file, ts).is_err() {
                    errs += 1;
                }
            } else if opts.verbose {
                println!("{} : correct", file);
            }
        } else if opts.verbose {
            println!("{} : no source file", file);
        }
    }

    if errs == 0 {
        Ok(())
    } else {
        Err(anyhow!("Failed to set times in {} files", errs))
    }
}

fn set_timestamp(file: &Utf8PathBuf, ts: SystemTime) -> io::Result<()> {
    let mtime = FileTime::from_system_time(ts);
    File::open(file)?;
    set_file_times(file, mtime, mtime)
}

fn format_time(time: SystemTime) -> String {
    let datetime = OffsetDateTime::from(time);
    datetime.format(&Rfc2822).unwrap()
}

fn timestamps_for(dir: &Utf8PathBuf, opts: &Opts) -> MTimeMap {
    if opts.verbose {
        println!("Collecting timestamps for {}", dir);
    }

    let pattern = format!("{}/**/*", dir);
    glob(&pattern)
        .expect("Failed to read glob pattern")
        .filter_map(Result::ok)
        .filter_map(|path| {
            let metadata = metadata(&path).ok()?;
            let modified_time = metadata.modified().ok()?;
            let relative_path = path.strip_prefix(dir).ok()?;
            let utf8_path = Utf8Path::from_path(relative_path).unwrap();
            Some((utf8_path.to_path_buf(), modified_time))
        })
        .collect()
}

fn main() {
    let cli = Cli::parse();

    let opts = Opts {
        verbose: cli.verbose,
        noop: cli.noop,
    };

    match touch_directory(
        &Utf8PathBuf::from(cli.source),
        &Utf8PathBuf::from(cli.dest),
        &opts,
    ) {
        Ok(_) => std::process::exit(0),
        Err(e) => {
            eprintln!("ERROR: {}", e);
            std::process::exit(1);
        }
    }
}

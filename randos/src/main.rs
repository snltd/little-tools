mod utils;
use crate::utils::types::{ActionOpts, FilterOpts};
use crate::utils::{actions, dir, filter, namer};
use camino::Utf8PathBuf;
use clap::Parser;
use rand::{seq::SliceRandom, thread_rng};
use regex::Regex;
use time::{Duration, OffsetDateTime};

#[derive(Parser, Debug)]
#[clap(version, about = "Links to (semi-) random files", long_about = None)]
struct Cli {
    /// Only consider source files with these extensions (comma separated)
    #[clap(short, long)]
    ext_list: Option<String>,
    /// Only consider source files older than this many days
    #[clap(short = 'O', long)]
    older: Option<u16>,
    /// Only consider source files newer than this many days
    #[clap(short = 'N', long)]
    newer: Option<u16>,
    /// Only consider source files matching this Rust regex
    #[clap(short = 'x', long)]
    regex: Option<String>,
    /// Use relative paths
    #[clap(short = 'R', long)]
    relative: bool,
    /// How to name targets. One of plain, hash, sequential, expand. Defaults to plain
    #[clap(short, long)]
    scheme: Option<String>,
    /// Recurse down source directories
    #[clap(short, long)]
    recurse: bool,
    /// Be verbose
    #[clap(short, long)]
    verbose: bool,
    /// Say what would happen without doing it
    #[clap(short, long)]
    noop: bool,
    /// Action to perform: mv (move), cp (copy), ln (symlink), lnh (hardlink), mvx (cross-fs move)
    #[arg(required = true)]
    action: String,
    /// Operate on this many files
    #[arg(required = true)]
    count: usize,
    /// Source files and/or directories
    #[arg(required = true)]
    sources: Vec<String>,
    /// Destination directory
    #[arg(required = true)]
    dest_dir: String,
}

fn parse_extensions(cli_ext_list: Option<String>) -> Option<Vec<String>> {
    cli_ext_list.map(|str| str.split(',').map(|s| s.into()).collect())
}

fn die(message: String) -> ! {
    eprintln!("ERROR: {}", message);
    std::process::exit(1);
}

fn parse_regex(cli_regex: Option<String>) -> Option<Regex> {
    match cli_regex {
        Some(regex_str) => match Regex::new(&regex_str) {
            Ok(rx) => Some(rx),
            Err(_) => die(format!("Regex '{}' cannot be compiled", regex_str)),
        },
        None => None,
    }
}

fn parse_age(cli_age: Option<u16>) -> Option<u64> {
    match cli_age {
        Some(days) => {
            let now = OffsetDateTime::now_utc();
            let difference = Duration::days(days as i64);
            let cutoff = now.saturating_sub(difference).unix_timestamp();
            Some(cutoff as u64)
        }
        None => None,
    }
}

fn main() {
    let cli = Cli::parse();

    let dest_dir = match Utf8PathBuf::from(&cli.dest_dir).canonicalize_utf8() {
        Ok(dir) => dir,
        Err(_) => {
            eprintln!("ERROR: {} does not exist", cli.dest_dir);
            std::process::exit(1);
        }
    };

    let filter_opts = FilterOpts {
        extensions: parse_extensions(cli.ext_list),
        older: parse_age(cli.older),
        newer: parse_age(cli.newer),
        regex: parse_regex(cli.regex),
    };

    let cli_dirs = dir::pathbuf_set(&cli.sources);

    let candidate_pool = match dir::expand_file_list(&cli_dirs, cli.recurse) {
        Ok(list) => list,
        Err(e) => die(format!("could not generate candidate list: {}", e)),
    };

    let candidates = candidate_pool.len();

    let required_sources = if cli.count <= candidates {
        cli.count
    } else {
        candidates
    };

    let mut sources: Vec<(&Utf8PathBuf, Utf8PathBuf)> = Vec::new();
    let mut index_list: Vec<usize> = (0..candidates).collect();
    let mut rng = thread_rng();
    index_list.shuffle(&mut rng);
    let mut seq_no = 0;

    while let Some(index) = index_list.pop() {
        let candidate = &candidate_pool[index];
        if filter::is_candidate(candidate, &filter_opts) {
            if let Some(target_basename) = namer::name_from(candidate, seq_no, &cli.scheme) {
                sources.push((candidate, dest_dir.join(target_basename)));
                seq_no += 1;
                if sources.len() == required_sources {
                    break;
                }
            }
        }
    }

    if sources.len() < cli.count {
        println!(
            "WARNING: requested {} files, but {} suitable candidates were found",
            cli.count,
            sources.len()
        );
    }

    let action_opts = ActionOpts {
        action: cli.action,
        noop: cli.noop,
        relative_links: cli.relative,
        verbose: cli.verbose,
    };

    let mut exit_code = 0;

    for (source, dest) in sources {
        if let Err(e) = actions::run(source, &dest, &action_opts) {
            eprintln!("ERROR: {}", e);
            exit_code = 1;
        }
    }

    std::process::exit(exit_code);
}

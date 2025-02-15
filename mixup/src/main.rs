use anyhow::anyhow;
use camino::Utf8PathBuf;
use clap::Parser;
use rand::rng;
use rand::seq::SliceRandom;
use std::fs::File;
use std::io::{self, BufReader};

const GRANULARITIES: [&str; 4] = ["char", "all-words", "line-words", "line", "file"];

#[derive(Parser, Debug)]
#[clap(version, about = "Mixes up bodies of text", long_about = None)]
struct Cli {
    /// If multiple files are given, whether to randomly interleave them at the given granularity
    #[clap(short, long)]
    interleave: bool,
    /// Granularity of mixing. Can be none, char, word, line, or file
    #[arg(required = true)]
    granularity: String,
    /// Source file(s)
    #[arg(required = true)]
    sources: Vec<Utf8PathBuf>,
}

fn passthru_file(path: &Utf8PathBuf) -> anyhow::Result<()> {
    let fh = File::open(path)?;
    let mut reader = BufReader::new(fh);
    let stdout = io::stdout();
    let mut stdout_lock = stdout.lock();
    io::copy(&mut reader, &mut stdout_lock)?;
    Ok(())
}

fn process_files(files: &[Utf8PathBuf]) -> anyhow::Result<()> {
    let mut shuffled_files = files.to_vec();
    shuffled_files.shuffle(&mut rng());

    for f in shuffled_files {
        passthru_file(&f)?;
    }

    Ok(())
}

fn main() {
    let cli = Cli::parse();

    if cli.granularity == "file" {
        match process_files(&cli.sources) {
            Ok(_) => std::process::exit(0),
            Err(e) => {
                eprintln!("ERROR: {}", e);
                std::process::exit(1);
            }
        }
    }

    let raw_sources: Vec<String> = cli
        .sources
        .iter()
        .filter_map(|source_file| {
            let source_path = Utf8PathBuf::from(source_file);
            if source_path.exists() {
                std::fs::read_to_string(source_file).ok()
            } else {
                eprintln!("WARNING: '{}' not found", source_file);
                None
            }
        })
        .collect();

    // println!("{:?}", raw_sources);

    let mut chunked_sources = raw_sources.iter().map(|raw_source| {
        let mut in_bits = match cli.granularity.as_str() {
            // "char" => char_splitter(&raw_source),
            "all-words" => word_splitter(raw_source.as_str()),
            "line-words" => line_word_splitter(raw_source.as_str()),
            "line" => line_splitter(raw_source.as_str()),
            _ => {
                eprintln!("Granularity must be one of {}", GRANULARITIES.join(", "));
                std::process::exit(1);
            }
        };

        in_bits.shuffle(&mut rng());
        in_bits
    });

    if !cli.interleave {
        for source in chunked_sources {
            let joiner = match cli.granularity.as_str() {
                "word" => " ",
                "line" => "\n",
                _ => panic!("unknown granularity"),
            };
            println!("{}", source.join(joiner));
        }
    }

    // loop {
    //     let populated_vec = chunked_sources.iter().filter(|s| !s.is_empty());
    // }
    // turn every raw into a vec of granularity
    //
    // shuffle the vecs

    // If interleave is on
    //
    //     pop off a random vac until we can't do that any more
    // else
    //     collect() each vec
    //
}

// fn char_splitter(raw_source: &String) -> Vec<&str> {
//     raw_source.chars().map(|c| c.to_string()).collect()
// }

fn line_word_splitter(raw_source: &str) -> Vec<String> {
    let lines: Vec<_> = raw_source.lines().collect();

    lines
        .iter()
        .map(|l| {
            let mut words: Vec<_> = l.split_whitespace().collect();
            words.shuffle(&mut rng());
            words.join(" ")
        })
        .collect()
}

fn word_splitter(raw_source: &str) -> Vec<&str> {
    raw_source.split_whitespace().collect()
}

fn line_splitter(raw_source: &str) -> Vec<&str> {
    raw_source.lines().collect()
}

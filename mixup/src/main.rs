mod utils;

use crate::utils::{all_chars, all_words, files, line_words, lines};
use camino::Utf8PathBuf;
use clap::Parser;

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

fn main() {
    let cli = Cli::parse();
    let mut source_paths: Vec<Utf8PathBuf> = Vec::new();
    let mut read_errs = false;

    for source in cli.sources.iter() {
        match Utf8PathBuf::from(source).canonicalize_utf8() {
            Ok(p) => source_paths.push(p),
            Err(_) => {
                eprintln!("ERROR: could not read '{}'", source);
                read_errs = true;
            }
        }
    }

    if read_errs {
        std::process::exit(1);
    }

    if cli.granularity == "file" || cli.granularity == "files" {
        if cli.interleave {
            eprintln!("NOTICE: files always interleave");
        }

        match files::mixup_files(&source_paths) {
            Ok(_) => std::process::exit(0),
            Err(e) => {
                eprintln!("ERROR: {}", e);
                std::process::exit(1);
            }
        }
    }

    let raw_sources: Vec<String> = source_paths
        .iter()
        .filter_map(|source_file| std::fs::read_to_string(source_file).ok())
        .collect();

    let result = match cli.granularity.as_str() {
        "all-words" | "words" => all_words::mixup_words(raw_sources, cli.interleave),
        "chars" | "char" => all_chars::mixup_chars(raw_sources, cli.interleave),
        "line-words" => line_words::mixup_lines(raw_sources, cli.interleave),
        "line" | "lines" => lines::mixup_lines(raw_sources, cli.interleave),
        _ => {
            eprintln!("ERROR: granularity must be one of char, all-words, line-words, line, file");
            std::process::exit(2);
        }
    };

    match result {
        Ok(_) => std::process::exit(0),
        Err(e) => {
            eprintln!("ERROR: {}", e);
            std::process::exit(1);
        }
    }
}

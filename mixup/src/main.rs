use camino::Utf8PathBuf;
use clap::Parser;

const GRANULARITIES: [&str; 5] = ["none", "char", "word", "line", "file"];

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
    sources: Vec<String>,
}

fn main() {
    let cli = Cli::parse();

    if !GRANULARITIES.contains(&cli.granularity.as_str()) {
        eprintln!("Granularity must be one of {}", GRANULARITIES.join(", "));
        std::process::exit(1);
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

    println!("{:?}", raw_sources);
}

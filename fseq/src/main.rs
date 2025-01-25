use crate::utils::types::Opts;
use clap::{Args, Parser, Subcommand};

mod subcommands;
#[macro_use]
mod utils;

const THE_TAG: &str = "raw";

#[derive(Parser)]
#[clap(version, about = "Sequences file names", long_about = None)]
struct Cli {
    /// Identification tag
    #[clap(short = 't', long = "tag", default_value = THE_TAG)]
    tag: String,
    /// Print what would happen, without doing it
    #[clap(short, long)]
    noop: bool,
    /// Be explicit about all operations
    #[clap(short, long)]
    verbose: bool,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Commands which operate on one or more directories
    Dir(DirArgs),
    /// Commands which operate on one or more files
    File(FileArgs),
}

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
struct DirArgs {
    #[command(subcommand)]
    /// consolidate the sequencing in a directory
    command: Option<DirCommands>,
}

#[derive(Debug, Subcommand)]
enum DirCommands {
    /// Renames all files sequentially to <dir_name>.[tag.]<number>.suffix
    Consolidate { dir: Vec<String> },
    /// Renumbers files which match the naming scheme in order of modification time  
    NumByAge { dir: Vec<String> },
}

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
struct FileArgs {
    #[command(subcommand)]
    command: Option<FileCommands>,
}

#[derive(Debug, Subcommand)]
enum FileCommands {
    /// Flips the presence of the filename tag
    Flip { flist: Vec<String> },
    /// Sets the filename tag if it is not set already
    Set { flist: Vec<String> },
    /// Removes any filename tag
    Unset { flist: Vec<String> },
}

fn main() {
    let cli = Cli::parse();

    let opts = Opts {
        verbose: cli.verbose,
        noop: cli.noop,
        tag: cli.tag.clone(),
    };

    if let Err(e) = run_command(cli, opts) {
        eprintln!("ERROR main: {}", e);
        std::process::exit(1);
    }
}

fn run_command(cli: Cli, opts: Opts) -> Result<(), Box<dyn std::error::Error>> {
    match cli.command {
        Commands::Dir(dir) => match dir.command {
            Some(dir_cmd) => match dir_cmd {
                DirCommands::Consolidate { dir } => {
                    Ok(subcommands::dir_consolidate::run(&dir, opts)?)
                }
                DirCommands::NumByAge { dir } => Ok(subcommands::dir_num_by_age::run(&dir, opts)?),
            },
            None => {
                eprintln!("ERROR: the 'dir' command needs a subcommand.");
                std::process::exit(1);
            }
        },
        Commands::File(file) => match file.command {
            Some(file_cmd) => match file_cmd {
                FileCommands::Flip { flist } => Ok(subcommands::file_flip::run(&flist, opts)?),
                FileCommands::Set { flist } => Ok(subcommands::file_set::run(&flist, opts)?),
                FileCommands::Unset { flist } => Ok(subcommands::file_unset::run(&flist, opts)?),
            },
            None => {
                eprintln!("ERROR: the 'file' command needs a subcommand.");
                std::process::exit(1);
            }
        },
    }
}

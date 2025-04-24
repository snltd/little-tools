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
    Consolidate {
        #[arg(required = true)]
        dirs: Vec<String>,
    },
    /// Renumbers files which match the naming scheme in order of modification time  
    NumByAge {
        #[arg(required = true)]
        dirs: Vec<String>,
    },
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
    #[command(alias = "flip-tag")]
    Flip {
        #[arg(required = true)]
        files: Vec<String>,
    },
    /// Sets the filename tag if it is not set already
    #[command(alias = "set-tag")]
    Set {
        #[arg(required = true)]
        files: Vec<String>,
    },
    /// Removes any filename tag
    #[command(alias = "unset-tag")]
    Unset {
        #[arg(required = true)]
        files: Vec<String>,
    },
}

fn main() {
    let cli = Cli::parse();

    let opts = Opts {
        verbose: cli.verbose,
        noop: cli.noop,
        tag: cli.tag.clone(),
    };

    let result = match cli.command {
        Commands::Dir(dir) => match dir.command {
            Some(dir_cmd) => match dir_cmd {
                DirCommands::Consolidate { dirs } => subcommands::dir_consolidate::run(&dirs, opts),
                DirCommands::NumByAge { dirs } => subcommands::dir_num_by_age::run(&dirs, opts),
            },
            None => {
                eprintln!("ERROR: the 'dir' command needs a subcommand.");
                std::process::exit(2);
            }
        },
        Commands::File(file) => match file.command {
            Some(file_cmd) => match file_cmd {
                FileCommands::Flip { files } => subcommands::file_flip::run(&files, opts),
                FileCommands::Set { files } => subcommands::file_set::run(&files, opts),
                FileCommands::Unset { files } => subcommands::file_unset::run(&files, opts),
            },
            None => {
                eprintln!("ERROR: the 'file' command needs a subcommand.");
                std::process::exit(2);
            }
        },
    };

    match result {
        Ok(_) => std::process::exit(0),
        Err(_) => std::process::exit(1),
    }
}

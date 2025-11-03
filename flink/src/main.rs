use anyhow::Context;
use camino::{Utf8Path, Utf8PathBuf};
use clap::Parser;
use common::{if_op, verbose};
use std::{env, fs, process};

struct Opts {
    noop: bool,
    verbose: bool,
    root: Utf8PathBuf,
    force: bool,
}

/// Links files from one or more source directories into a single target directory.
#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Cli {
    /// Be verbose
    #[arg(short, long)]
    verbose: bool,
    /// Say what would happen without doing it
    #[arg(short, long)]
    noop: bool,
    /// Force re-creation of links. Files will be left in place and flagged
    #[arg(short, long)]
    force: bool,
    /// Root (target) dir
    #[arg(short = 'R', long, default_value_t = default_home())]
    root: Utf8PathBuf,
    #[arg(required = true)]
    /// Paths to dotfile directories
    source_dir: Vec<Utf8PathBuf>,
}

fn default_home() -> Utf8PathBuf {
    let home = env::var("HOME").expect("cannot get $HOME");
    Utf8PathBuf::from(home)
}

fn main() {
    let cli = Cli::parse();
    let mut exit_code = 0;

    let opts = Opts {
        verbose: cli.verbose,
        noop: cli.noop,
        root: cli.root,
        force: cli.force,
    };

    for dir in cli.source_dir {
        if dir.exists() {
            match link_from_dir(&dir, &opts) {
                Ok(_) => (),
                Err(e) => {
                    eprintln!("ERROR linking from {dir}: {e}");
                    exit_code = 1;
                }
            }
        } else {
            eprintln!("ERROR: {dir} not found");
            exit_code = 1;
        }
    }

    process::exit(exit_code)
}

fn link_from_dir(dir: &Utf8Path, opts: &Opts) -> anyhow::Result<()> {
    for file in dir.read_dir_utf8()? {
        let entry = file?;
        let link_to = entry.path();
        let raw_fname = link_to.file_name().context("cannot get name of {path}")?;

        if raw_fname.starts_with('.') || raw_fname.starts_with("README") {
            continue;
        }

        let target_relative_path = format!(".{}", raw_fname.replace('-', "/"));
        let link_from = opts.root.join(&target_relative_path);

        let relative_link =
            pathdiff::diff_utf8_paths(link_to, link_from.parent().context("could not get parent")?)
                .context("cannot calculate relative link")?;

        if link_from.is_symlink() {
            if opts.force {
                if_op!(opts, fs::remove_file(&link_from)).context("failed to remove file")?;
            } else {
                continue;
            }
        }

        if link_from.exists() {
            println!("{link_from} is a file");
            continue;
        }

        verbose!(opts, "{link_from} -> {relative_link}");

        if target_relative_path.contains("/") {
            let link_parent = link_from.parent().context("could not get parent")?;

            if !link_parent.exists() {
                verbose!(opts, "creating {link_parent}");
                if_op!(opts, fs::create_dir_all(link_parent))?;
            }
        }

        if_op!(opts, std::os::unix::fs::symlink(&relative_link, link_from))?;
    }

    Ok(())
}

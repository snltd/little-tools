use crate::utils::types::ActionOpts;
use camino::Utf8PathBuf;
use pathdiff::diff_utf8_paths;
use std::{fs, io};

pub fn run(source: &Utf8PathBuf, dest: &Utf8PathBuf, opts: &ActionOpts) -> io::Result<()> {
    if opts.verbose || opts.noop {
        println!("{}: {} -> {}", opts.action, source, dest);
    }

    if opts.noop {
        return Ok(());
    }

    match opts.action.as_str() {
        "cp" | "copy" => fs::copy(source, dest).map(|_| ()),
        "mv" | "move" => fs::rename(source, dest),
        "mvx" | "movexfs" => movexfs(source, dest),
        "lnh" | "hardlink" => fs::hard_link(source, dest),
        _ => symlink(source, dest, opts.relative_links),
    }
}

fn symlink(source: &Utf8PathBuf, target: &Utf8PathBuf, relative: bool) -> io::Result<()> {
    let source = if relative {
        println!("doing relative link");
        match diff_utf8_paths(source, target) {
            Some(path) => path,
            None => source.clone(),
        }
    } else {
        source.clone()
    };

    println!("{} -> {}", target, source);

    std::os::unix::fs::symlink(source, target)
}

fn movexfs(source: &Utf8PathBuf, dest: &Utf8PathBuf) -> io::Result<()> {
    fs::copy(source, dest).map(|_| ())?;
    fs::remove_file(source)
}

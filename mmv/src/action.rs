use super::GlobalOpts;
use anyhow::{Context, anyhow};
use camino::Utf8Path;
use common::verbose;
use std::fs;

pub fn rename_file(src: &Utf8Path, target: &Utf8Path, opts: &GlobalOpts) -> anyhow::Result<bool> {
    let (source_name, target_name) = if opts.full {
        (src.to_string(), target.to_string())
    } else {
        (
            src.file_name()
                .with_context(|| format!("cannot get file name of {src}"))?
                .into(),
            target
                .file_name()
                .with_context(|| format!("cannot get file name of {src}"))?
                .into(),
        )
    };

    if target_name == source_name {
        verbose!(opts, "{}: no change", source_name);
        Ok(false)
    } else if opts.git {
        println!("git mv {} {}", src, target);
        Ok(true)
    } else {
        if opts.terse {
            println!("{}", target_name);
        } else {
            verbose!(opts, "{} -> {}", source_name, target_name);
        }

        rename(src, target, opts)?;
        Ok(true)
    }
}

fn rename(src: &Utf8Path, dest: &Utf8Path, opts: &GlobalOpts) -> anyhow::Result<()> {
    if dest.exists() && !opts.clobber {
        Err(anyhow!("filename collision [-c to clobber]"))
    } else if !opts.noop {
        Ok(fs::rename(src, dest)?)
    } else {
        Ok(())
    }
}

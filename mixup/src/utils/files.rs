use camino::Utf8PathBuf;
use rand::rng;
use rand::seq::SliceRandom;
use std::fs::File;
use std::io::{self, BufReader};

pub fn mixup_files(files: &[Utf8PathBuf]) -> anyhow::Result<()> {
    let mut shuffled_files = files.to_vec();
    shuffled_files.shuffle(&mut rng());

    for f in shuffled_files {
        passthru_file(&f)?;
    }

    Ok(())
}

fn passthru_file(path: &Utf8PathBuf) -> anyhow::Result<()> {
    let fh = File::open(path)?;
    let mut reader = BufReader::new(fh);
    let stdout = io::stdout();
    let mut stdout_lock = stdout.lock();
    io::copy(&mut reader, &mut stdout_lock)?;
    Ok(())
}

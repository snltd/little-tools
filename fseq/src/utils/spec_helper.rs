use crate::utils::file_tokens::FileTokens;
use std::env::current_dir;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

pub fn fixture(dir: &str) -> PathBuf {
    current_dir().unwrap().join("tests/resources").join(dir)
}

pub fn file_token_with_time(file: &Path, ts: SystemTime) -> (PathBuf, FileTokens) {
    let mut tokens = FileTokens::new(file, "tag").unwrap();
    tokens.mtime = ts;
    (file.to_owned(), tokens)
}

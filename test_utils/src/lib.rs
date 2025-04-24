use assert_fs::fixture::PathChild;
use assert_fs::prelude::*;
use assert_fs::TempDir;
use std::env::current_dir;
use std::fs;
use std::path::PathBuf;

pub fn fixture(file: &str) -> PathBuf {
    current_dir().unwrap().join("tests/resources").join(file)
}

pub fn fixture_as_string(file: &str) -> String {
    fixture(file).to_string_lossy().to_string()
}

pub fn sample_output(file: &str) -> String {
    let file = current_dir().unwrap().join("tests/outputs").join(file);
    fs::read_to_string(file).unwrap()
}

pub fn fixture_dir(dir_name: &str, files: Vec<&str>) -> (TempDir, PathBuf) {
    let temp = TempDir::new().expect("failed to create temp dir");
    let dir = temp.child(dir_name);
    dir.create_dir_all().expect("failed to create subdirectory");

    for file in files {
        let file_path = dir.child(file);
        file_path.write_str("").expect("failed to create file");
    }

    let canon_dir = std::fs::canonicalize(dir.path()).expect("canonicalize failed");

    (temp, canon_dir)
}

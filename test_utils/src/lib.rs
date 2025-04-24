use assert_fs::fixture::PathChild;
use assert_fs::prelude::*;
use assert_fs::TempDir;
use camino::Utf8PathBuf;
use std::env::current_dir;
use std::fs;

pub fn fixture(file: &str) -> Utf8PathBuf {
    let pb = current_dir().unwrap().join("tests/resources").join(file);
    Utf8PathBuf::from_path_buf(pb).unwrap()
}

pub fn fixture_as_string(file: &str) -> String {
    fixture(file).to_string()
}

pub fn sample_output(file: &str) -> String {
    let file = current_dir().unwrap().join("tests/outputs").join(file);
    fs::read_to_string(file).unwrap()
}

pub fn fixture_dir(dir_name: &str, files: Vec<&str>) -> (TempDir, Utf8PathBuf) {
    let temp = TempDir::new().expect("failed to create temp dir");
    let dir = temp.child(dir_name);
    dir.create_dir_all().expect("failed to create subdirectory");

    for file in files {
        let file_path = dir.child(file);
        file_path.write_str(file).expect("failed to create file");
    }

    let pb = dir.path().to_path_buf();
    let canon_dir = Utf8PathBuf::from_path_buf(pb)
        .unwrap()
        .canonicalize_utf8()
        .unwrap();

    (temp, canon_dir)
}

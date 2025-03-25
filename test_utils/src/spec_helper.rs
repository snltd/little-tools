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

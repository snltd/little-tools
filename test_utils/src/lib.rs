use camino::Utf8PathBuf;
use camino_tempfile_ext::prelude::*;
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

pub fn fixture_dir(dir_name: &str, files: Vec<&str>) -> (Utf8TempDir, Utf8PathBuf) {
    let temp = Utf8TempDir::new().expect("failed to create temp dir");
    let dir = temp.child(dir_name);
    dir.create_dir_all().expect("failed to create subdirectory");

    for file in files {
        let file_path = dir.child(file);
        file_path.write_str(file).expect("failed to create file");
    }

    (temp, dir.canonicalize_utf8().unwrap())
}

pub fn setup_randos_source_dir() -> Utf8TempDir {
    let src_dir = Utf8TempDir::new().unwrap();
    let f1 = src_dir.child("file_1.sfx");
    let f2 = src_dir.child("file_2.sfx");
    let f3 = src_dir.child("file_3.sfx");
    f1.write_str("file1").unwrap();
    f2.write_str("file2").unwrap();
    f3.write_str("file3").unwrap();

    src_dir
}

pub trait ContainsFiles {
    fn contains_files(&self, count: usize) -> bool;
}

impl ContainsFiles for Utf8TempDir {
    fn contains_files(&self, count: usize) -> bool {
        match self.path().is_dir() {
            true => {
                let files_in_dir = self.path().read_dir().expect("cannot read dir").count();
                files_in_dir == count
            }
            false => false,
        }
    }
}

use camino::Utf8PathBuf;
use std::env::current_dir;

pub fn fixture(dir: &str) -> Utf8PathBuf {
    let base_dir = current_dir().unwrap();

    Utf8PathBuf::from_path_buf(base_dir)
        .unwrap()
        .join("test/resources")
        .join(dir)
}

use camino::Utf8PathBuf;
use std::env;

pub fn fixture(file: &str) -> Utf8PathBuf {
    let pwd = env::current_dir().unwrap();

    let mut ret = Utf8PathBuf::from_path_buf(pwd).unwrap();

    ret.push("tests");
    ret.push("resources");
    ret.push(file);

    ret
}

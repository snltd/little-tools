use camino_tempfile_ext::prelude::*;

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

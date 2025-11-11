use assert_cmd::cargo::cargo_bin_cmd;
use camino_tempfile_ext::prelude::*;
use predicates::prelude::*;
use std::fs;
use std::time::{Duration, SystemTime};

#[test]
#[ignore]
fn test_align_mtimes() {
    let src = Utf8TempDir::new().unwrap();
    let dst = Utf8TempDir::new().unwrap();

    let src_file = src.child("file1.txt");
    let dst_file = dst.child("file1.txt");

    src_file.touch().unwrap();
    dst_file.touch().unwrap();

    let old_time = SystemTime::now() - Duration::from_secs(3600);
    let new_time = SystemTime::now();

    filetime::set_file_mtime(
        src_file.as_path(),
        filetime::FileTime::from_system_time(new_time),
    )
    .unwrap();

    filetime::set_file_mtime(
        dst_file.as_path(),
        filetime::FileTime::from_system_time(old_time),
    )
    .unwrap();

    cargo_bin_cmd!("align-mtimes")
        .arg(src.path())
        .arg(dst.path())
        .assert()
        .success();

    let src_meta = fs::metadata(src_file.as_path()).unwrap();
    let dst_meta = fs::metadata(dst_file.as_path()).unwrap();

    let src_mtime = src_meta.modified().unwrap();
    let dst_mtime = dst_meta.modified().unwrap();

    assert_eq!(src_mtime, dst_mtime, "Modification times do not match");
}

#[test]
#[ignore]
fn test_align_mtimes_noop() {
    let src = Utf8TempDir::new().unwrap();
    let dst = Utf8TempDir::new().unwrap();

    let src_file = src.child("file1.txt");
    let dst_file = dst.child("file1.txt");

    src_file.touch().unwrap();
    dst_file.touch().unwrap();

    let old_time = SystemTime::now() - Duration::from_secs(3600);
    let new_time = SystemTime::now();

    filetime::set_file_mtime(
        src_file.as_path(),
        filetime::FileTime::from_system_time(new_time),
    )
    .unwrap();

    filetime::set_file_mtime(
        dst_file.as_path(),
        filetime::FileTime::from_system_time(old_time),
    )
    .unwrap();

    cargo_bin_cmd!("align-mtimes")
        .arg("--noop")
        .arg(src.path())
        .arg(dst.path())
        .assert()
        .success();

    let dst_meta = fs::metadata(dst_file.as_path()).unwrap();
    let dst_mtime = dst_meta.modified().unwrap();

    assert_eq!(old_time, dst_mtime, "Modification times do not match");
}

#[test]
#[ignore]
fn test_align_mtimes_bad_usage() {
    cargo_bin_cmd!("align-mtimes")
        .arg("/tmp")
        .assert()
        .stderr(predicate::str::contains(
            "the following required arguments were not provided",
        ))
        .failure();

    cargo_bin_cmd!("align-mtimes")
        .assert()
        .stderr(predicate::str::contains(
            "the following required arguments were not provided",
        ))
        .failure();

    cargo_bin_cmd!("align-mtimes")
        .args(["/no/such/dir", "/no/such/other/dir"])
        .assert()
        .stderr("ERROR: No source directory: /no/such/dir\n")
        .failure();
}

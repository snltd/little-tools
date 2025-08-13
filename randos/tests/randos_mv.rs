#[cfg(test)]
mod test {
    use assert_cmd::Command;
    use assert_fs::prelude::*;

    fn setup_source_dir() -> assert_fs::TempDir {
        let src_dir = assert_fs::TempDir::new().unwrap();
        let f1 = src_dir.child("file_1.sfx");
        let f2 = src_dir.child("file_2.sfx");
        let f3 = src_dir.child("file_3.sfx");
        f1.write_str("file1").unwrap();
        f2.write_str("file2").unwrap();
        f3.write_str("file3").unwrap();

        src_dir
    }

    #[test]
    #[ignore]
    fn test_randos_mv_noop() {
        let src_dir = setup_source_dir();
        let target_dir = assert_fs::TempDir::new().unwrap();

        Command::cargo_bin("randos")
            .unwrap()
            .arg("mv")
            .arg("-r")
            .arg("2")
            .arg("--noop")
            .arg(src_dir.to_string_lossy().as_ref())
            .arg(target_dir.to_string_lossy().as_ref())
            .assert()
            .success();

        assert_eq!(3, src_dir.read_dir().unwrap().count());
        assert_eq!(0, target_dir.read_dir().unwrap().count());
    }

    #[test]
    #[ignore]
    fn test_randos_plain_mv() {
        let src_dir = setup_source_dir();
        let target_dir = assert_fs::TempDir::new().unwrap();

        Command::cargo_bin("randos")
            .unwrap()
            .arg("mv")
            .arg("-r")
            .arg("2")
            .arg(src_dir.to_string_lossy().as_ref())
            .arg(target_dir.to_string_lossy().as_ref())
            .assert()
            .success();

        assert_eq!(1, src_dir.read_dir().unwrap().count());
        assert_eq!(2, target_dir.read_dir().unwrap().count());
    }

    #[test]
    #[ignore]
    fn test_randos_plain_mv_no_matching_suffix() {
        let src_dir = setup_source_dir();
        let target_dir = assert_fs::TempDir::new().unwrap();

        Command::cargo_bin("randos")
            .unwrap()
            .arg("mv")
            .arg("-r")
            .arg("-e")
            .arg("txt")
            .arg("2")
            .arg(src_dir.to_string_lossy().as_ref())
            .arg(target_dir.to_string_lossy().as_ref())
            .assert()
            .success();

        assert_eq!(3, src_dir.read_dir().unwrap().count());
        assert_eq!(0, target_dir.read_dir().unwrap().count());
    }

    #[test]
    #[ignore]
    fn test_randos_mv_scheme_not_recursive() {
        let src_dir = setup_source_dir();
        let target_dir = assert_fs::TempDir::new().unwrap();

        Command::cargo_bin("randos")
            .unwrap()
            .arg("mv")
            .arg("--scheme")
            .arg("hash")
            .arg("1")
            .arg(src_dir.join("file_1.sfx").to_string_lossy().as_ref())
            .arg(src_dir.join("file_2.sfx").to_string_lossy().as_ref())
            .arg(src_dir.join("file_3.sfx").to_string_lossy().as_ref())
            .arg(target_dir.to_string_lossy().as_ref())
            .assert()
            .success();

        assert_eq!(2, src_dir.read_dir().unwrap().count());
        assert_eq!(1, target_dir.read_dir().unwrap().count());
    }

    #[test]
    #[ignore]
    fn test_randos_plain_mv_regex() {
        let src_dir = setup_source_dir();
        let target_dir = assert_fs::TempDir::new().unwrap();

        Command::cargo_bin("randos")
            .unwrap()
            .arg("mv")
            .arg("-r")
            .arg("-x")
            .arg("file_2")
            .arg("4")
            .arg(src_dir.to_string_lossy().as_ref())
            .arg(target_dir.to_string_lossy().as_ref())
            .assert()
            .success();

        assert_eq!(1, target_dir.read_dir().unwrap().count());
        assert!(target_dir.join("file_2.sfx").exists());
        assert!(!src_dir.join("file_2.sfx").exists());
    }

    #[test]
    #[ignore]
    fn test_mv_missing_file() {
        Command::cargo_bin("randos")
            .unwrap()
            .arg("mv")
            .arg("-r")
            .arg("4")
            .arg("/no/such/source")
            .arg("/no/such/target")
            .assert()
            .failure()
            .stderr("ERROR: /no/such/target does not exist\n");
    }
}

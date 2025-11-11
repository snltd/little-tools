#[cfg(test)]
mod test {
    use assert_cmd::cargo::cargo_bin_cmd;
    use camino_tempfile_ext::prelude::*;
    use std::fs;
    use test_utils::{ContainsFiles, setup_randos_source_dir};

    #[test]
    #[ignore]
    fn test_randos_ln_noop() {
        let src_dir = setup_randos_source_dir();
        let target_dir = Utf8TempDir::new().unwrap();

        cargo_bin_cmd!("randos")
            .arg("ln")
            .arg("-r")
            .arg("2")
            .arg("--noop")
            .arg(src_dir.path())
            .arg(target_dir.path())
            .assert()
            .success();

        assert!(target_dir.contains_files(0));
    }

    #[test]
    #[ignore]
    fn test_randos_plain_ln_absolute() {
        let src_dir = setup_randos_source_dir();
        let target_dir = Utf8TempDir::new().unwrap();

        cargo_bin_cmd!("randos")
            .arg("ln")
            .arg("-r")
            .arg("4")
            .arg(src_dir.path())
            .arg(target_dir.path())
            .assert()
            .success();

        assert!(target_dir.contains_files(3));
        assert!(target_dir.path().join("file_1.sfx").exists());
        assert!(target_dir.path().join("file_2.sfx").exists());
        assert!(target_dir.path().join("file_3.sfx").exists());
        assert_eq!(
            "file1",
            fs::read_to_string(target_dir.path().join("file_1.sfx")).unwrap()
        );
    }

    #[test]
    #[ignore]
    fn test_randos_plain_ln_relative() {
        let src_dir = setup_randos_source_dir();
        let target_dir = Utf8TempDir::new().unwrap();

        cargo_bin_cmd!("randos")
            .arg("ln")
            .arg("-r")
            .arg("-R")
            .arg("4")
            .arg(src_dir.path())
            .arg(target_dir.path())
            .assert()
            .success();

        assert!(target_dir.contains_files(3));
        assert!(target_dir.path().join("file_1.sfx").exists());
        assert!(target_dir.path().join("file_2.sfx").exists());
        assert!(target_dir.path().join("file_3.sfx").exists());
        assert_eq!(
            "file1",
            fs::read_to_string(target_dir.path().join("file_1.sfx")).unwrap()
        );
    }

    #[test]
    #[ignore]
    fn test_randos_plain_ln_no_matching_suffix() {
        let src_dir = setup_randos_source_dir();
        let target_dir = Utf8TempDir::new().unwrap();

        cargo_bin_cmd!("randos")
            .arg("ln")
            .arg("-r")
            .arg("-e")
            .arg("txt")
            .arg("2")
            .arg(src_dir.path())
            .arg(target_dir.path())
            .assert()
            .success();

        assert!(target_dir.contains_files(0));
    }

    #[test]
    #[ignore]
    fn test_randos_ln_scheme_not_recursive() {
        let src_dir = setup_randos_source_dir();
        let target_dir = Utf8TempDir::new().unwrap();

        cargo_bin_cmd!("randos")
            .arg("ln")
            .arg("--scheme")
            .arg("hash")
            .arg("1")
            .arg(src_dir.path().join("file_1.sfx"))
            .arg(src_dir.path().join("file_2.sfx"))
            .arg(src_dir.path().join("file_3.sfx"))
            .arg(target_dir.path())
            .assert()
            .success();

        assert!(target_dir.contains_files(1));
    }

    #[test]
    #[ignore]
    fn test_randos_plain_ln_regex() {
        let src_dir = setup_randos_source_dir();
        let target_dir = Utf8TempDir::new().unwrap();

        cargo_bin_cmd!("randos")
            .arg("ln")
            .arg("-r")
            .arg("-x")
            .arg("file_2")
            .arg("4")
            .arg(src_dir.path())
            .arg(target_dir.path())
            .assert()
            .success();

        assert!(target_dir.contains_files(1));
        assert!(target_dir.path().join("file_2.sfx").exists());
    }

    #[test]
    #[ignore]
    fn test_ln_missing_file() {
        cargo_bin_cmd!("randos")
            .arg("ln")
            .arg("-r")
            .arg("4")
            .arg("/no/such/source")
            .arg("/no/such/target")
            .assert()
            .failure()
            .stderr("ERROR: /no/such/target does not exist\n");
    }
}

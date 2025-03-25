#[cfg(test)]
mod test {
    use assert_cmd::Command;
    use assert_fs::prelude::*;
    use predicates::prelude::*;

    #[test]
    #[ignore]
    fn test_cs_command_noop() {
        let tmp = assert_fs::TempDir::new().unwrap();
        let f1 = tmp.child("File$.sfx");
        let f2 = tmp.child("File$$.sfx");
        let f3 = tmp.child("File$$$.sfx");

        f1.touch().unwrap();
        f2.touch().unwrap();
        f3.touch().unwrap();

        Command::cargo_bin("cs")
            .unwrap()
            .args([
                "--noop",
                f1.to_string_lossy().as_ref(),
                f2.to_string_lossy().as_ref(),
                f3.to_string_lossy().as_ref(),
            ])
            .assert()
            .success();

        assert!(f1.exists());
        assert!(f2.exists());
        assert!(f3.exists());
    }

    #[test]
    #[ignore]
    fn test_cs_command_renumbers() {
        let tmp = assert_fs::TempDir::new().unwrap();
        let f1 = tmp.child("File$.sfx");
        let f2 = tmp.child("File$$.sfx");
        let f3 = tmp.child("File$$$.sfx");

        f1.touch().unwrap();
        f2.touch().unwrap();
        f3.touch().unwrap();

        Command::cargo_bin("cs")
            .unwrap()
            .args([
                f1.to_string_lossy().as_ref(),
                f2.to_string_lossy().as_ref(),
                f3.to_string_lossy().as_ref(),
            ])
            .assert()
            .stdout("")
            .success();

        assert!(!f1.exists());
        assert!(!f2.exists());
        assert!(!f3.exists());

        assert!(tmp.join("file.sfx").exists());
        assert!(tmp.join("file.001.sfx").exists());
        assert!(tmp.join("file.002.sfx").exists());
    }

    #[test]
    #[ignore]
    fn test_cs_command_renumbers_verbose() {
        let tmp = assert_fs::TempDir::new().unwrap();
        let f1 = tmp.child("File$.sfx");
        let f2 = tmp.child("File$$.sfx");
        let f3 = tmp.child("File$$$.sfx");

        f1.touch().unwrap();
        f2.touch().unwrap();
        f3.touch().unwrap();

        Command::cargo_bin("cs")
            .unwrap()
            .args([
                "--verbose",
                f1.to_string_lossy().as_ref(),
                f2.to_string_lossy().as_ref(),
                f3.to_string_lossy().as_ref(),
            ])
            .assert()
            .success()
            .stdout(predicate::str::contains(format!(
                "{} -> {}/file.sfx",
                f1.display(),
                tmp.display()
            )))
            .stdout(predicate::str::contains(format!(
                "{} -> {}/file.001.sfx",
                f2.display(),
                tmp.display()
            )))
            .stdout(predicate::str::contains(format!(
                "{} -> {}/file.002.sfx",
                f3.display(),
                tmp.display()
            )));

        assert!(!f1.exists());
        assert!(!f2.exists());
        assert!(!f3.exists());

        assert!(tmp.join("file.sfx").exists());
        assert!(tmp.join("file.001.sfx").exists());
        assert!(tmp.join("file.002.sfx").exists());
    }

    #[test]
    #[ignore]
    fn test_cs_command_clobbers() {
        let tmp = assert_fs::TempDir::new().unwrap();
        let f1 = tmp.child("File$.sfx");
        let f2 = tmp.child("File$$.sfx");
        let f3 = tmp.child("File$$$.sfx");

        f1.touch().unwrap();
        f2.touch().unwrap();
        f3.touch().unwrap();

        Command::cargo_bin("cs")
            .unwrap()
            .args([
                "--clobber",
                f1.to_string_lossy().as_ref(),
                f2.to_string_lossy().as_ref(),
                f3.to_string_lossy().as_ref(),
            ])
            .assert()
            .success();

        assert!(!f1.exists());
        assert!(!f2.exists());
        assert!(!f3.exists());

        assert!(tmp.join("file.sfx").exists());
        assert!(!tmp.join("file.001.sfx").exists());
        assert!(!tmp.join("file.002.sfx").exists());
    }

    #[test]
    #[ignore]
    fn test_cs_command_nonumber() {
        let tmp = assert_fs::TempDir::new().unwrap();
        let f1 = tmp.child("File$.sfx");
        let f2 = tmp.child("File$$.sfx");
        let f3 = tmp.child("File$$$.sfx");

        f1.touch().unwrap();
        f2.touch().unwrap();
        f3.touch().unwrap();

        Command::cargo_bin("cs")
            .unwrap()
            .args([
                "--nonumber",
                f1.to_string_lossy().as_ref(),
                f2.to_string_lossy().as_ref(),
                f3.to_string_lossy().as_ref(),
            ])
            .assert()
            .failure();

        assert!(!f1.exists());
        assert!(f2.exists());
        assert!(f3.exists());
        assert!(tmp.join("file.sfx").exists());
    }

    #[test]
    #[ignore]
    fn test_cs_no_args() {
        Command::cargo_bin("cs")
            .unwrap()
            .assert()
            .failure()
            .stderr(predicate::str::contains(
                "the following required arguments were not provided",
            ));
    }

    #[test]
    #[ignore]
    fn test_cs_missing_file() {
        Command::cargo_bin("cs")
            .unwrap()
            .arg("/no/such/file")
            .assert()
            .failure()
            .stderr("ERROR on /no/such/file: file not found\n");
    }
}

#[cfg(test)]
mod test {
    use assert_cmd::cargo::cargo_bin_cmd;
    use camino_tempfile_ext::prelude::*;
    use predicates::prelude::*;

    #[test]
    #[ignore]
    fn test_cs_command_noop() {
        let tmp = Utf8TempDir::new().unwrap();
        let f1 = tmp.child("File$.sfx");
        let f2 = tmp.child("File$$.sfx");
        let f3 = tmp.child("File$$$.sfx");

        f1.touch().unwrap();
        f2.touch().unwrap();
        f3.touch().unwrap();

        cargo_bin_cmd!("cs")
            .arg("--noop")
            .arg(f1.as_path())
            .arg(f2.as_path())
            .arg(f3.as_path())
            .assert()
            .success();

        assert!(f1.exists());
        assert!(f2.exists());
        assert!(f3.exists());
    }

    #[test]
    #[ignore]
    fn test_cs_command_renumbers() {
        let tmp = Utf8TempDir::new().unwrap();
        let f1 = tmp.child("File$.sfx");
        let f2 = tmp.child("File$$.sfx");
        let f3 = tmp.child("File$$$.sfx");

        f1.touch().unwrap();
        f2.touch().unwrap();
        f3.touch().unwrap();

        cargo_bin_cmd!("cs")
            .arg(f1.as_path())
            .arg(f2.as_path())
            .arg(f3.as_path())
            .assert()
            .stdout("")
            .success();

        assert!(!f1.exists());
        assert!(!f2.exists());
        assert!(!f3.exists());

        assert!(tmp.path().join("file.sfx").exists());
        assert!(tmp.path().join("file.001.sfx").exists());
        assert!(tmp.path().join("file.002.sfx").exists());
    }

    #[test]
    #[ignore]
    fn test_cs_command_renumbers_verbose() {
        let tmp = Utf8TempDir::new().unwrap();
        let f1 = tmp.child("File$.sfx");
        let f2 = tmp.child("File$$.sfx");
        let f3 = tmp.child("File$$$.sfx");

        f1.touch().unwrap();
        f2.touch().unwrap();
        f3.touch().unwrap();

        let f1 = f1.canonicalize_utf8().unwrap();
        let f2 = f2.canonicalize_utf8().unwrap();
        let f3 = f3.canonicalize_utf8().unwrap();
        let tmp = tmp.path().canonicalize_utf8().unwrap();

        cargo_bin_cmd!("cs")
            .arg("--verbose")
            .arg(&f1)
            .arg(&f2)
            .arg(&f3)
            .assert()
            .success()
            .stdout(predicate::str::contains(format!("{f1} -> {tmp}/file.sfx",)))
            .stdout(predicate::str::contains(format!(
                "{f2} -> {tmp}/file.001.sfx",
            )))
            .stdout(predicate::str::contains(format!(
                "{f3} -> {tmp}/file.002.sfx",
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
        let tmp = Utf8TempDir::new().unwrap();
        let f1 = tmp.child("File$.sfx");
        let f2 = tmp.child("File$$.sfx");
        let f3 = tmp.child("File$$$.sfx");

        f1.touch().unwrap();
        f2.touch().unwrap();
        f3.touch().unwrap();

        cargo_bin_cmd!("cs")
            .arg("--clobber")
            .arg(f1.as_path())
            .arg(f2.as_path())
            .arg(f3.as_path())
            .assert()
            .success();

        assert!(!f1.exists());
        assert!(!f2.exists());
        assert!(!f3.exists());

        assert!(tmp.path().join("file.sfx").exists());
        assert!(!tmp.path().join("file.001.sfx").exists());
        assert!(!tmp.path().join("file.002.sfx").exists());
    }

    #[test]
    #[ignore]
    fn test_cs_command_nonumber() {
        let tmp = Utf8TempDir::new().unwrap();
        let f1 = tmp.child("File$.sfx");
        let f2 = tmp.child("File$$.sfx");
        let f3 = tmp.child("File$$$.sfx");

        f1.touch().unwrap();
        f2.touch().unwrap();
        f3.touch().unwrap();

        cargo_bin_cmd!("cs")
            .arg("--nonumber")
            .arg(f1.as_path())
            .arg(f2.as_path())
            .arg(f3.as_path())
            .assert()
            .failure();

        assert!(!f1.exists());
        assert!(f2.exists());
        assert!(f3.exists());
        assert!(tmp.path().join("file.sfx").exists());
    }

    #[test]
    #[ignore]
    fn test_cs_no_args() {
        cargo_bin_cmd!("cs")
            .assert()
            .failure()
            .stderr(predicate::str::contains(
                "the following required arguments were not provided",
            ));
    }

    #[test]
    #[ignore]
    fn test_cs_missing_file() {
        cargo_bin_cmd!("cs")
            .arg("/no/such/file")
            .assert()
            .failure()
            .stderr("ERROR on /no/such/file: file not found\n");
    }
}

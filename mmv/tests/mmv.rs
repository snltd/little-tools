#[cfg(test)]
mod test {
    use assert_cmd::Command;
    use predicates::prelude::*;
    use test_utils::fixture_dir;

    #[test]
    #[ignore]
    fn test_mmv_no_collisions() {
        let (_tmp, test_dir) = fixture_dir("fseq.test", vec!["before_001.txt", "before_002.txt"]);

        let before_1 = test_dir.join("before_001.txt");
        let before_2 = test_dir.join("before_002.txt");

        let after_1 = test_dir.join("after_001.txt");
        let after_2 = test_dir.join("after_002.txt");

        assert!(before_1.exists());
        assert!(before_2.exists());
        assert!(!after_1.exists());
        assert!(!after_2.exists());

        Command::cargo_bin("mmv")
            .unwrap()
            .arg("--verbose")
            .arg("before")
            .arg("after")
            .arg(&before_1)
            .arg(&before_2)
            .assert()
            .success()
            .stdout("before_001.txt -> after_001.txt\nbefore_002.txt -> after_002.txt\n");

        assert!(!before_1.exists());
        assert!(!before_2.exists());
        assert!(after_1.exists());
        assert!(after_2.exists());
    }

    #[test]
    #[ignore]
    fn test_mmv_collisions() {
        let (_tmp, test_dir) = fixture_dir(
            "fseq.test",
            vec![
                "before_001.txt",
                "before_002.txt",
                "after_001.txt",
                "after_002.txt",
            ],
        );

        let before_1 = test_dir.join("before_001.txt");
        let before_2 = test_dir.join("before_002.txt");

        let after_1 = test_dir.join("after_001.txt");
        let after_2 = test_dir.join("after_002.txt");

        assert!(before_1.exists());
        assert!(before_2.exists());
        assert!(after_1.exists());
        assert!(after_2.exists());

        Command::cargo_bin("mmv")
            .unwrap()
            .arg("--verbose")
            .arg("before")
            .arg("after")
            .arg(test_dir.join("before_001.txt"))
            .arg(test_dir.join("before_002.txt"))
            .assert()
            .failure();

        assert!(before_1.exists());
        assert!(before_2.exists());
        assert!(after_1.exists());
        assert!(after_2.exists());
    }

    #[test]
    #[ignore]
    fn test_mmv_collisions_clobber() {
        let (_tmp, test_dir) = fixture_dir(
            "fseq.test",
            vec![
                "before_001.txt",
                "before_002.txt",
                "after_001.txt",
                "after_002.txt",
            ],
        );

        let before_1 = test_dir.join("before_001.txt");
        let before_2 = test_dir.join("before_002.txt");

        let after_1 = test_dir.join("after_001.txt");
        let after_2 = test_dir.join("after_002.txt");

        assert!(before_1.exists());
        assert!(before_2.exists());
        assert!(after_1.exists());
        assert!(after_2.exists());

        Command::cargo_bin("mmv")
            .unwrap()
            .arg("--clobber")
            .arg("before")
            .arg("after")
            .arg(test_dir.join("before_001.txt"))
            .arg(test_dir.join("before_002.txt"))
            .assert()
            .success();

        assert!(!before_1.exists());
        assert!(!before_2.exists());
        assert!(after_1.exists());
        assert!(after_2.exists());
    }

    #[test]
    #[ignore]
    fn test_mmv_not_enough_args() {
        Command::cargo_bin("mmv")
            .unwrap()
            .assert()
            .failure()
            .stderr(predicate::str::contains(
                "the following required arguments were not provided",
            ));

        Command::cargo_bin("mmv")
            .unwrap()
            .arg("find")
            .assert()
            .failure()
            .stderr(predicate::str::contains(
                "the following required arguments were not provided",
            ));

        Command::cargo_bin("mmv")
            .unwrap()
            .arg("find")
            .arg("replace")
            .assert()
            .failure()
            .stderr(predicate::str::contains(
                "the following required arguments were not provided",
            ));
    }

    #[test]
    #[ignore]
    fn test_mmv_missing_file() {
        Command::cargo_bin("mmv")
            .unwrap()
            .arg("find")
            .arg("replace")
            .arg("/no/such/file")
            .assert()
            .failure()
            .stderr("ERROR: /no/such/file: No such file or directory (os error 2)\n");
    }
}

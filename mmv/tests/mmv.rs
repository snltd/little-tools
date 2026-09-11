#[cfg(test)]
mod test {
    use assert_cmd::cargo::cargo_bin_cmd;
    use snltest::fixture_dir;

    #[test]
    #[ignore]
    fn test_mmv_no_collisions() {
        let (_tmp, test_dir) = fixture_dir("mmv.test", vec!["before_001.txt", "before_002.txt"]);

        let before_1 = test_dir.join("before_001.txt");
        let before_2 = test_dir.join("before_002.txt");

        let after_1 = test_dir.join("after_001.txt");
        let after_2 = test_dir.join("after_002.txt");

        assert!(before_1.exists());
        assert!(before_2.exists());
        assert!(!after_1.exists());
        assert!(!after_2.exists());

        cargo_bin_cmd!("mmv")
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
            "mmv.test",
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

        cargo_bin_cmd!("mmv")
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
    fn test_mmv_collisions_wildcard() {
        let (_tmp, test_dir) = fixture_dir(
            "mmv.test",
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

        cargo_bin_cmd!("mmv")
            .arg("--verbose")
            .arg("before")
            .arg("after")
            .arg("*")
            .assert()
            .failure();

        assert!(before_1.exists());
        assert!(before_2.exists());
        assert!(after_1.exists());
        assert!(after_2.exists());
    }

    #[test]
    #[ignore]
    fn test_mmv_collisions_recode() {
        let (_tmp, test_dir) = fixture_dir(
            "mmv.test",
            vec![
                "file_1.mkv",
                "file_1.recoded.mkv",
                "file_2.mkv",
                "file_2.recoded.mkv",
            ],
        );

        let wanted_1 = test_dir.join("file_1.mkv");
        let wanted_2 = test_dir.join("file_2.mkv");

        let unwanted_1 = test_dir.join("file_1.recoded.mkv");
        let unwanted_2 = test_dir.join("file_2.recoded.mkv");

        assert!(wanted_1.exists());
        assert!(wanted_2.exists());
        assert!(unwanted_1.exists());
        assert!(unwanted_2.exists());

        cargo_bin_cmd!("mmv")
            .arg("--clobber")
            .arg(".recoded")
            .arg("")
            .arg(&wanted_1)
            .arg(&wanted_2)
            .arg(&unwanted_1)
            .arg(&unwanted_2)
            .assert()
            .success()
            .stdout("");

        assert!(wanted_1.exists());
        assert!(wanted_2.exists());
        assert!(!unwanted_1.exists());
        assert!(!unwanted_2.exists());
    }

    #[test]
    #[ignore]
    fn test_mmv_collisions_clobber() {
        let (_tmp, test_dir) = fixture_dir(
            "mmv.test",
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

        cargo_bin_cmd!("mmv")
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
}

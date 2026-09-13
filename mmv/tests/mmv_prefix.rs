#[cfg(test)]
mod test {
    use assert_cmd::cargo::cargo_bin_cmd;
    use snltest::fixture_dir;

    #[test]
    fn test_prefix() {
        let (_tmp, test_dir) = fixture_dir("mmv.test", vec!["file1.txt", "file2.txt"]);

        let before_1 = test_dir.join("file1.txt");
        let before_2 = test_dir.join("file2.txt");

        let after_1 = test_dir.join("new_file1.txt");
        let after_2 = test_dir.join("new_file2.txt");

        assert!(before_1.exists());
        assert!(before_2.exists());
        assert!(!after_1.exists());
        assert!(!after_2.exists());

        cargo_bin_cmd!("mmv")
            .arg("--prefix")
            .arg("new_")
            .arg(test_dir.join("file1.txt"))
            .arg(test_dir.join("file2.txt"))
            .assert()
            .success();

        assert!(!before_1.exists());
        assert!(!before_2.exists());
        assert!(after_1.exists());
        assert!(after_2.exists());
    }

    #[test]
    fn test_suffix() {
        let (_tmp, test_dir) = fixture_dir("mmv.test", vec!["file1.txt", "file2.txt"]);

        let before_1 = test_dir.join("file1.txt");
        let before_2 = test_dir.join("file2.txt");

        let after_1 = test_dir.join("file1_new.txt");
        let after_2 = test_dir.join("file2.txt");

        assert!(before_1.exists());
        assert!(before_2.exists());
        assert!(!after_1.exists());

        cargo_bin_cmd!("mmv")
            .arg("--suffix")
            .arg("_new")
            .arg(test_dir.join("file1.txt"))
            .assert()
            .success();

        assert!(!before_1.exists());
        assert!(after_1.exists());
        assert!(after_2.exists());
    }
}

#[cfg(test)]
mod test {
    use assert_cmd::cargo::cargo_bin_cmd;
    use snltest::fixture_dir;

    #[test]
    #[ignore]
    fn test_increment_first_number() {
        let (_tmp, test_dir) = fixture_dir("mmv.test", vec!["file1.txt", "file2.txt"]);
        let before_1 = test_dir.join("file1.txt");
        let before_2 = test_dir.join("file2.txt");

        let after_1 = test_dir.join("file3.txt");
        let after_2 = test_dir.join("file4.txt");

        assert!(before_1.exists());
        assert!(before_2.exists());
        assert!(!after_1.exists());
        assert!(!after_2.exists());

        cargo_bin_cmd!("mmv")
            .arg("--bump-number=0")
            .arg("2")
            .arg(&before_1)
            //     .arg(&before_2)
            .assert()
            .success();

        // assert!(!before_1.exists());
        // assert!(!before_2.exists());
        // assert!(after_1.exists());
        // assert!(after_2.exists());
    }
}

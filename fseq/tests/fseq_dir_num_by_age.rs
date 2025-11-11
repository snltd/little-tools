#[cfg(test)]
mod test {
    use assert_cmd::cargo::cargo_bin_cmd;
    use predicates::prelude::*;
    use test_utils::fixture_dir;

    #[test]
    #[ignore]
    fn test_fseq_num_by_age_same_names() {
        let original_names = vec![
            "fseq.test.0001.txt",
            "fseq.test.0005.txt",
            "fseq.test.0006.txt",
            "fseq.test.0003.txt",
            "fseq.test.TAG.0009.txt",
            "fseq.test.TAG.1009.txt",
        ];

        let expected_names = vec![
            "fseq.test.0001.txt",
            "fseq.test.0002.txt",
            "fseq.test.0003.txt",
            "fseq.test.0004.txt",
            "fseq.test.TAG.0001.txt",
            "fseq.test.TAG.0002.txt",
        ];

        let (_tmp, test_dir) = fixture_dir("fseq.test", original_names);

        cargo_bin_cmd!("fseq")
            .arg("--tag=TAG")
            .arg("dir")
            .arg("num-by-age")
            .arg(&test_dir)
            .assert()
            .success();

        assert_eq!(6, test_dir.read_dir().unwrap().count());

        for file in expected_names {
            assert!(test_dir.join(file).exists());
        }

        assert_eq!(
            "fseq.test.0001.txt".to_owned(),
            std::fs::read_to_string(test_dir.join("fseq.test.0001.txt")).unwrap()
        );

        assert_eq!(
            "fseq.test.0005.txt".to_owned(),
            std::fs::read_to_string(test_dir.join("fseq.test.0002.txt")).unwrap()
        );

        assert_eq!(
            "fseq.test.0006.txt".to_owned(),
            std::fs::read_to_string(test_dir.join("fseq.test.0003.txt")).unwrap()
        );

        assert_eq!(
            "fseq.test.0003.txt".to_owned(),
            std::fs::read_to_string(test_dir.join("fseq.test.0004.txt")).unwrap()
        );

        assert_eq!(
            "fseq.test.TAG.0009.txt".to_owned(),
            std::fs::read_to_string(test_dir.join("fseq.test.TAG.0001.txt")).unwrap()
        );

        assert_eq!(
            "fseq.test.TAG.1009.txt".to_owned(),
            std::fs::read_to_string(test_dir.join("fseq.test.TAG.0002.txt")).unwrap()
        );
    }

    #[test]
    #[ignore]
    fn test_fseq_dir_num_by_age_no_args() {
        cargo_bin_cmd!("fseq")
            .arg("dir")
            .arg("num-by-age")
            .assert()
            .failure()
            .stderr(predicate::str::contains(
                "the following required arguments were not provided",
            ));
    }

    #[test]
    #[ignore]
    fn test_fseq_dir_num_by_age_missing_file() {
        cargo_bin_cmd!("fseq")
            .arg("dir")
            .arg("num-by-age")
            .arg("/no/such/dir")
            .assert()
            .failure()
            .stderr("ERROR: No such file or directory (os error 2)\n");
    }
}

#[cfg(test)]
mod test {
    use assert_cmd::Command;
    use predicates::prelude::*;
    use test_utils::fixture_dir;

    #[test]
    #[ignore]
    fn test_fseq_consolidate_same_names() {
        let original_names = vec![
            "fseq.test.0001.txt",
            "fseq.test.0003.txt",
            "fseq.test.0005.txt",
            "fseq.test.0006.txt",
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

        Command::cargo_bin("fseq")
            .unwrap()
            .arg("--tag=TAG")
            .arg("dir")
            .arg("consolidate")
            .arg(&test_dir)
            .assert()
            .success();

        assert_eq!(6, test_dir.read_dir().unwrap().count());

        for file in expected_names {
            assert!(test_dir.join(file).exists());
        }
    }

    #[test]
    #[ignore]
    fn test_fseq_consolidate_different_names() {
        let original_names = vec![
            "file.txt",
            "fseq.test.0003.txt",
            "merp-merp.txt",
            "line.006.txt",
            "merp.TAG.0009.txt",
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

        Command::cargo_bin("fseq")
            .unwrap()
            .arg("--tag=TAG")
            .arg("dir")
            .arg("consolidate")
            .arg(&test_dir)
            .assert()
            .success();

        assert_eq!(6, test_dir.read_dir().unwrap().count());

        for file in expected_names {
            assert!(test_dir.join(file).exists());
        }
    }

    #[test]
    #[ignore]
    fn test_fseq_dir_consolidate_no_args() {
        Command::cargo_bin("fseq")
            .unwrap()
            .arg("dir")
            .arg("consolidate")
            .assert()
            .failure()
            .stderr(predicate::str::contains(
                "the following required arguments were not provided",
            ));
    }

    #[test]
    #[ignore]
    fn test_fseq_dir_consolidate_missing_file() {
        Command::cargo_bin("fseq")
            .unwrap()
            .arg("dir")
            .arg("consolidate")
            .arg("/no/such/dir")
            .assert()
            .failure()
            .stderr("ERROR: No such file or directory (os error 2)\n");
    }
}

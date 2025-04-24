#[cfg(test)]
mod test {
    use assert_cmd::Command;
    use predicates::prelude::*;
    use test_utils::fixture_dir;

    #[test]
    #[ignore]
    fn test_fseq_file_flip() {
        let original_file = "fseq.test.0001.txt";
        let expected_file = "fseq.test.TAG.0002.txt";

        let (_tmp, test_dir) =
            fixture_dir("fseq.test", vec![original_file, "fseq.test.TAG.0001.txt"]);

        Command::cargo_bin("fseq")
            .unwrap()
            .arg("--tag=TAG")
            .arg("file")
            .arg("flip")
            .arg(test_dir.join(original_file))
            .assert()
            .success();

        assert!(test_dir.join(expected_file).exists());
        assert!(!test_dir.join(original_file).exists());

        Command::cargo_bin("fseq")
            .unwrap()
            .arg("--tag=TAG")
            .arg("file")
            .arg("flip")
            .arg(test_dir.join(expected_file))
            .assert()
            .success();

        assert!(!test_dir.join(expected_file).exists());
        assert!(test_dir.join(original_file).exists());
    }

    #[test]
    #[ignore]
    fn test_fseq_file_flip_no_args() {
        Command::cargo_bin("fseq")
            .unwrap()
            .arg("file")
            .arg("flip")
            .assert()
            .failure()
            .stderr(predicate::str::contains(
                "the following required arguments were not provided",
            ));
    }

    #[test]
    #[ignore]
    fn test_fseq_file_flip_missing_file() {
        Command::cargo_bin("fseq")
            .unwrap()
            .arg("file")
            .arg("flip")
            .arg("/no/such/file")
            .assert()
            .failure()
            .stderr("ERROR on /no/such/file: No such file or directory (os error 2)\n");
    }
}

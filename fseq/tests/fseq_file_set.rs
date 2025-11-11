#[cfg(test)]
mod test {
    use assert_cmd::cargo::cargo_bin_cmd;
    use predicates::prelude::*;
    use test_utils::fixture_dir;

    #[test]
    #[ignore]
    fn test_fseq_file_set() {
        let untagged_file = "fseq.test.0001.txt";
        let tagged_file = "fseq.test.TAG.0001.txt";
        let new_tagged_file = "fseq.test.TAG.0002.txt";
        let (_tmp, test_dir) = fixture_dir("fseq.test", vec![tagged_file, untagged_file]);

        cargo_bin_cmd!("fseq")
            .arg("--tag=TAG")
            .arg("file")
            .arg("set")
            .arg(test_dir.join(tagged_file))
            .assert()
            .success();

        assert!(test_dir.join(tagged_file).exists());
        assert!(test_dir.join(untagged_file).exists());
        assert!(!test_dir.join(new_tagged_file).exists());

        cargo_bin_cmd!("fseq")
            .arg("--tag=TAG")
            .arg("file")
            .arg("set")
            .arg(test_dir.join(untagged_file))
            .assert()
            .success();

        assert!(!test_dir.join(untagged_file).exists());
        assert!(test_dir.join(tagged_file).exists());
        assert!(test_dir.join(new_tagged_file).exists());
    }

    #[test]
    #[ignore]
    fn test_fseq_file_set_no_args() {
        cargo_bin_cmd!("fseq")
            .arg("file")
            .arg("set")
            .assert()
            .failure()
            .stderr(predicate::str::contains(
                "the following required arguments were not provided",
            ));
    }

    #[test]
    #[ignore]
    fn test_fseq_file_set_missing_file() {
        cargo_bin_cmd!("fseq")
            .arg("file")
            .arg("set")
            .arg("/no/such/file")
            .assert()
            .failure()
            .stderr("ERROR on /no/such/file: No such file or directory (os error 2)\n");
    }
}

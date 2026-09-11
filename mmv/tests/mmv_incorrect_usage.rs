#[cfg(test)]
mod test {
    use assert_cmd::cargo::cargo_bin_cmd;
    use predicates::prelude::*;

    #[test]
    #[ignore]
    fn test_mmv_not_enough_args() {
        cargo_bin_cmd!("mmv")
            .assert()
            .failure()
            .stderr(predicate::str::contains(
                "the following required arguments were not provided",
            ));

        cargo_bin_cmd!("mmv")
            .arg("find")
            .assert()
            .failure()
            .stderr(predicate::str::contains(
                "the following required arguments were not provided",
            ));

        cargo_bin_cmd!("mmv")
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
        cargo_bin_cmd!("mmv")
            .arg("find")
            .arg("replace")
            .arg("/no/such/file")
            .assert()
            .failure()
            .stderr("ERROR: cannot canonicalize /no/such/file: No such file or directory (os error 2)\n");
    }
}

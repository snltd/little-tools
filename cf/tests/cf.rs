#[cfg(test)]
mod test {
    use assert_cmd::cargo::cargo_bin_cmd;

    #[ignore]
    #[test]
    fn test_cf_missing_directory() {
        cargo_bin_cmd!("cf")
            .arg("/no/such/file")
            .assert()
            .failure()
            .stderr("ERROR: /no/such/file is not a directory\n");
    }

    #[ignore]
    #[test]
    fn test_cf_not_a_directory() {
        cargo_bin_cmd!("cf")
            .arg("./Cargo.toml")
            .assert()
            .failure()
            .stderr("ERROR: ./Cargo.toml is not a directory\n");
    }

    #[ignore]
    #[test]
    fn test_cf() {
        cargo_bin_cmd!("cf")
            .arg("src")
            .assert()
            .success()
            .stdout("\t1\tsrc\n");
    }
}

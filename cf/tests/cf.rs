#[cfg(test)]
mod test {
    use assert_cmd::Command;

    #[ignore]
    #[test]
    fn test_cf_missing_directory() {
        Command::cargo_bin("cf")
            .unwrap()
            .arg("/no/such/file")
            .assert()
            .failure()
            .stderr("ERROR: /no/such/file is not a directory\n");
    }

    #[ignore]
    #[test]
    fn test_cf_not_a_directory() {
        Command::cargo_bin("cf")
            .unwrap()
            .arg("./Cargo.toml")
            .assert()
            .failure()
            .stderr("ERROR: ./Cargo.toml is not a directory\n");
    }

    #[ignore]
    #[test]
    fn test_cf() {
        Command::cargo_bin("cf")
            .unwrap()
            .arg("src")
            .assert()
            .success()
            .stdout("\t1\tsrc\n");
    }
}

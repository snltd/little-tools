#[cfg(test)]
mod test {
    use assert_cmd::cargo::cargo_bin_cmd;
    use std::collections::HashSet;
    use test_utils::fixture_as_string;

    #[test]
    fn test_single_file_is_like_cat() {
        cargo_bin_cmd!("mixup")
            .args(["file", &fixture_as_string("files/f1")])
            .assert()
            .success()
            .stdout("f1\n")
            .stderr("");
    }

    #[test]
    fn test_files_get_mixed_up() {
        let mut outputs: HashSet<String> = HashSet::from([
            "f1\nf2\nf3\n".into(),
            "f1\nf3\nf2\n".into(),
            "f2\nf1\nf3\n".into(),
            "f2\nf3\nf1\n".into(),
            "f3\nf1\nf2\n".into(),
            "f3\nf2\nf1\n".into(),
        ]);

        let mut tries = 1000;
        let mut found_all_combos = false;

        loop {
            let mut cmd = cargo_bin_cmd!("mixup");

            cmd.arg("file");
            cmd.arg(fixture_as_string("files/f1"));
            cmd.arg(fixture_as_string("files/f2"));
            cmd.arg(fixture_as_string("files/f3"));

            cmd.assert().success();

            let output = String::from_utf8(cmd.output().unwrap().stdout).unwrap();

            if outputs.contains(&output) {
                outputs.remove(&output);
            }

            if outputs.is_empty() {
                found_all_combos = true;
                break;
            }

            tries -= 1;

            if tries == 0 {
                break;
            }
        }

        assert!(found_all_combos);
    }

    #[test]
    fn test_warning_of_interleave() {
        cargo_bin_cmd!("mixup")
            .args(["file", "--interleave", &fixture_as_string("files/f1")])
            .assert()
            .success()
            .stderr("NOTICE: files always interleave\n");
    }

    #[test]
    fn test_error_on_missing_file() {
        cargo_bin_cmd!("mixup")
            .args(["file", "/file/does/not/exist"])
            .assert()
            .failure()
            .code(1)
            .stdout("")
            .stderr("ERROR: could not read '/file/does/not/exist'\n");
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;
    use test_utils::fixture_as_string;

    #[test]
    fn test_single_file_is_like_cat() {
        assert_cmd::Command::cargo_bin("mixup")
            .unwrap()
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
            let mut cmd = assert_cmd::Command::cargo_bin("mixup").unwrap();

            cmd.args([
                "file",
                &fixture_as_string("files/f1"),
                &fixture_as_string("files/f2"),
                &fixture_as_string("files/f3"),
            ]);

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
        assert_cmd::Command::cargo_bin("mixup")
            .unwrap()
            .args(["file", "--interleave", &fixture_as_string("files/f1")])
            .assert()
            .success()
            .stderr("NOTICE: files always interleave\n");
    }

    #[test]
    fn test_error_on_missing_file() {
        assert_cmd::Command::cargo_bin("mixup")
            .unwrap()
            .args(["file", "/file/does/not/exist"])
            .assert()
            .failure()
            .code(1)
            .stdout("")
            .stderr("ERROR: could not read '/file/does/not/exist'\n");
    }
}

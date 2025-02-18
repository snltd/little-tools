#[cfg(test)]
mod test {
    use std::collections::HashSet;
    use test_utils::spec_helper::fixture_as_string;

    #[test]
    fn test_all_words_with_interleave_get_mixed_up() {
        let mut outputs: HashSet<String> = HashSet::from([
            "a1 a2 a3 b1 b2\n".into(),
            "a2 b2 a3 a1 b1\n".into(),
            "b1 a2 a3 b2 a1\n".into(),
            "b2 b1 a3 a1 a2\n".into(),
        ]);

        let mut tries = 1000;
        let mut found_all_combos = false;

        loop {
            let mut cmd = assert_cmd::Command::cargo_bin("mixup").unwrap();

            cmd.args([
                "all-words",
                "-i",
                &fixture_as_string("all_words/f1"),
                &fixture_as_string("all_words/f2"),
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
    fn test_all_words_without_interleave_get_mixed_up() {
        let mut outputs: HashSet<String> = HashSet::from([
            "a1 a2 a3\nb1 b2\n".into(),
            "a2 a1 a3\nb2 b1\n".into(),
            "a3 a2 a1\nb2 b1\n".into(),
        ]);

        let mut tries = 200;
        let mut found_all_combos = false;

        loop {
            let mut cmd = assert_cmd::Command::cargo_bin("mixup").unwrap();

            cmd.args([
                "all-words",
                &fixture_as_string("all_words/f1"),
                &fixture_as_string("all_words/f2"),
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
    fn test_all_words_error_on_missing_file() {
        assert_cmd::Command::cargo_bin("mixup")
            .unwrap()
            .args(["all-words", "/file/does/not/exist"])
            .assert()
            .failure()
            .code(1)
            .stdout("")
            .stderr("ERROR: could not read '/file/does/not/exist'\n");
    }
}

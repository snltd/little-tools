#[cfg(test)]
mod test {
    use std::collections::HashSet;
    use test_utils::spec_helper::fixture_as_string;

    #[test]
    fn test_lines_with_interleave_get_mixed_up() {
        // Not exhaustive but makes the point
        let mut outputs: HashSet<String> = HashSet::from([
            "abc def\nghi jkl\n123 456\n789\n".into(),
            "ghi jkl\n123 456\nabc def\n789\n".into(),
            "abc def\n123 456\n789\nghi jkl\n".into(),
            "789\n123 456\nabc def\nghi jkl\n".into(),
        ]);

        let mut tries = 200;
        let mut found_all_combos = false;

        loop {
            let mut cmd = assert_cmd::Command::cargo_bin("mixup").unwrap();

            cmd.args([
                "lines",
                "-i",
                &fixture_as_string("lines/f1"),
                &fixture_as_string("lines/f2"),
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
    fn test_lines_without_interleave_get_mixed_up() {
        let mut outputs: HashSet<String> = HashSet::from([
            "abc def\nghi jkl\n123 456\n789\n".into(),
            "abc def\nghi jkl\n789\n123 456\n".into(),
            "ghi jkl\nabc def\n123 456\n789\n".into(),
            "ghi jkl\nabc def\n789\n123 456\n".into(),
        ]);

        let mut tries = 200;
        let mut found_all_combos = false;

        loop {
            let mut cmd = assert_cmd::Command::cargo_bin("mixup").unwrap();

            cmd.args([
                "lines",
                &fixture_as_string("lines/f1"),
                &fixture_as_string("lines/f2"),
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
    fn test_lines_error_on_missing_file() {
        assert_cmd::Command::cargo_bin("mixup")
            .unwrap()
            .args(["lines", "/file/does/not/exist"])
            .assert()
            .failure()
            .code(1)
            .stdout("")
            .stderr("ERROR: could not read '/file/does/not/exist'\n");
    }
}

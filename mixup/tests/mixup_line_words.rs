#[cfg(test)]
mod test {
    use assert_cmd::cargo::cargo_bin_cmd;
    use std::collections::HashSet;
    use test_utils::fixture_as_string;

    #[test]
    fn test_line_words_with_interleave_get_mixed_up() {
        let mut outputs: HashSet<String> = HashSet::from([
            "abc def\nghi jkl\n123 456\n789\n".into(),
            "def abc\nghi jkl\n123 456\n789\n".into(),
            "ghi jkl\n456 123\ndef abc\n789\n".into(),
            "456 123\ndef abc\nghi jkl\n789\n".into(),
            "789\nabc def\njkl ghi\n456 123\n".into(),
            "123 456\ndef abc\n789\njkl ghi\n".into(),
        ]);

        let mut tries = 3000;
        let mut found_all_combos = false;

        loop {
            let mut cmd = cargo_bin_cmd!("mixup");
            cmd.arg("line-words");
            cmd.arg("-i");
            cmd.arg(fixture_as_string("line_words/f1"));
            cmd.arg(fixture_as_string("line_words/f2"));
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
    fn test_line_words_without_interleave_get_mixed_up() {
        let mut outputs: HashSet<String> = HashSet::from([
            "abc def\nghi jkl\n123 456\n789\n".into(),
            "def abc\nghi jkl\n123 456\n789\n".into(),
            "def abc\nghi jkl\n456 123\n789\n".into(),
            "abc def\njkl ghi\n123 456\n789\n".into(),
            "abc def\njkl ghi\n456 123\n789\n".into(),
            "def abc\njkl ghi\n123 456\n789\n".into(),
            "def abc\njkl ghi\n456 123\n789\n".into(),
        ]);

        let mut tries = 200;
        let mut found_all_combos = false;

        loop {
            let mut cmd = cargo_bin_cmd!("mixup");
            cmd.arg("line-words");
            cmd.arg(fixture_as_string("line_words/f1"));
            cmd.arg(fixture_as_string("line_words/f2"));
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
    fn test_line_words_error_on_missing_file() {
        cargo_bin_cmd!("mixup")
            .args(["line-words", "/file/does/not/exist"])
            .assert()
            .failure()
            .code(1)
            .stdout("")
            .stderr("ERROR: could not read '/file/does/not/exist'\n");
    }
}

use crate::utils::types::FilterOpts;
use camino::Utf8Path;
use std::fs;
use std::time::UNIX_EPOCH;

fn file_mtime(file: &Utf8Path) -> u64 {
    fs::metadata(file)
        .unwrap()
        .modified()
        .unwrap()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

pub fn is_candidate(file: &Utf8Path, opts: &FilterOpts) -> bool {
    if !file.is_file() {
        return false;
    }

    if let Some(extensions) = &opts.extensions {
        match file.extension() {
            Some(file_ext) => {
                if extensions.iter().all(|e| e != file_ext) {
                    return false;
                }
            }
            None => return false,
        }
    }

    if let Some(rx) = &opts.regex
        && let Some(file_name) = file.file_name()
        && !rx.is_match(file_name)
    {
        return false;
    }

    if let Some(timestamp) = &opts.older
        && file_mtime(file) > *timestamp
    {
        return false;
    }

    if let Some(timestamp) = &opts.newer
        && file_mtime(file) < *timestamp
    {
        return false;
    }

    true
}

#[cfg(test)]
mod test {
    use super::*;
    use filetime::{FileTime, set_file_times};
    use regex::Regex;
    use test_utils::fixture;

    #[test]
    fn test_is_candidates_age_filter() {
        let file_1_mtime = FileTime::from_unix_time(1737100000, 0);
        let file_2_mtime = FileTime::from_unix_time(1737200000, 0);
        let file_3_mtime = FileTime::from_unix_time(1737300000, 0);

        set_file_times(fixture("dir_2/file_2_1.txt"), file_1_mtime, file_1_mtime).unwrap();
        set_file_times(fixture("dir_2/file_2_2.txt"), file_2_mtime, file_2_mtime).unwrap();
        set_file_times(fixture("dir_2/file_2_3.txt"), file_3_mtime, file_3_mtime).unwrap();

        let selector_opts = FilterOpts {
            extensions: None,
            older: Some(1737240000),
            newer: None,
            regex: None,
        };

        let good_candidates = vec!["dir_2/file_2_1.txt", "dir_2/file_2_2.txt"];
        let bad_candidates = vec!["dir_2/file_2_3.txt"];

        test_candidates(good_candidates, bad_candidates, &selector_opts);

        let selector_opts = FilterOpts {
            extensions: None,
            older: None,
            newer: Some(1737240000),
            regex: None,
        };

        let good_candidates = vec!["dir_2/file_2_3.txt"];
        let bad_candidates = vec!["dir_2/file_2_1.txt", "dir_2/file_2_2.txt"];

        test_candidates(good_candidates, bad_candidates, &selector_opts);

        let selector_opts = FilterOpts {
            extensions: None,
            newer: Some(1737100010),
            older: Some(1737200001),
            regex: None,
        };

        let good_candidates = vec!["dir_2/file_2_2.txt"];
        let bad_candidates = vec!["dir_2/file_2_1.txt", "dir_2/file_2_3.txt"];

        test_candidates(good_candidates, bad_candidates, &selector_opts);
    }

    #[test]
    fn test_is_candidates_no_filter() {
        let selector_opts = FilterOpts {
            extensions: None,
            older: None,
            newer: None,
            regex: None,
        };

        let good_candidates = vec![
            "dir_1/file_1_4",
            "dir_1/file_1_3.png",
            "dir_1/file_1_1.sfx",
            "dir_1/file_1_2.sfx",
        ];

        let bad_candidates = vec!["dir_1/subdir_1_1"];

        test_candidates(good_candidates, bad_candidates, &selector_opts);
    }

    #[test]
    fn test_is_candidates_filter_extension() {
        let selector_opts = FilterOpts {
            extensions: Some(vec!["txt".into(), "png".into()]),
            older: None,
            newer: None,
            regex: None,
        };

        let good_candidates = vec!["dir_1/file_1_3.png"];

        let bad_candidates = vec![
            "dir_1/file_1_4",
            "dir_1/file_1_1.sfx",
            "dir_1/subdir_1_1",
            "dir_1/file_1_2.sfx",
        ];

        test_candidates(good_candidates, bad_candidates, &selector_opts);
    }

    #[test]
    fn test_is_candidates_regex() {
        let selector_opts = FilterOpts {
            extensions: None,
            older: None,
            newer: None,
            regex: Some(Regex::new("1_[23]").unwrap()),
        };

        let good_candidates = vec!["dir_1/file_1_3.png", "dir_1/file_1_2.sfx"];
        let bad_candidates = vec!["dir_1/subdir_1_1", "dir_1/file_1_1.sfx", "dir_1/file_1_4"];

        test_candidates(good_candidates, bad_candidates, &selector_opts);
    }

    #[test]
    fn test_is_candidates_regex_and_suffix() {
        let selector_opts = FilterOpts {
            extensions: Some(vec!["png".into()]),
            older: None,
            newer: None,
            regex: Some(Regex::new("1_[23]").unwrap()),
        };

        let good_candidates = vec!["dir_1/file_1_3.png"];
        let bad_candidates = vec![
            "dir_1/subdir_1_1",
            "dir_1/file_1_1.sfx",
            "dir_1/file_1_2.sfx",
            "dir_1/file_1_4",
        ];

        test_candidates(good_candidates, bad_candidates, &selector_opts);
    }

    #[test]
    fn test_is_candidates_no_regex_matches() {
        let selector_opts = FilterOpts {
            extensions: None,
            older: None,
            newer: None,
            regex: Some(Regex::new("xyz]").unwrap()),
        };

        let good_candidates = Vec::new();

        let bad_candidates = vec![
            "dir_1/file_1_3.png",
            "dir_1/file_1_1.sfx",
            "dir_1/file_1_4",
            "dir_1/subdir_1_1",
            "dir_1/file_1_2.sfx",
        ];

        test_candidates(good_candidates, bad_candidates, &selector_opts);
    }

    #[test]
    fn test_is_candidates_no_suffix_matches() {
        let selector_opts = FilterOpts {
            extensions: Some(vec!["merp".into(), "byerp".into()]),
            older: None,
            newer: None,
            regex: None,
        };

        let good_candidates = Vec::new();

        let bad_candidates = vec![
            "dir_1/file_1_3.png",
            "dir_1/file_1_1.sfx",
            "dir_1/subdir_1_1",
            "dir_1/file_1_4",
            "dir_1/file_1_2.sfx",
        ];

        test_candidates(good_candidates, bad_candidates, &selector_opts);
    }

    fn test_candidates(good: Vec<&str>, bad: Vec<&str>, opts: &FilterOpts) {
        good.iter()
            .for_each(|c| assert!(is_candidate(&fixture(c), opts), "{} WAS BAD", c));

        bad.iter()
            .for_each(|c| assert!(!is_candidate(&fixture(c), opts), "{} WAS GOOD", c));
    }
}

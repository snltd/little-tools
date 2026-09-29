use crate::types::{RenameAction, Replacements};
use anyhow::Context;
use camino::Utf8PathBuf;
use regex::Regex;

pub fn new_name(orig: &str, action: &RenameAction) -> anyhow::Result<String> {
    let ret = match action {
        RenameAction::ReplaceRegex(ropts) => match &ropts.replacements {
            Replacements::All => ropts.from.replacen(orig, 0, &ropts.to).to_string(),
            Replacements::Indices(indices) => replace_nth_rx(orig, &ropts.from, &ropts.to, indices),
        },
        RenameAction::ReplaceLiteral(ropts) => match &ropts.replacements {
            Replacements::All => orig.replace(&ropts.from, &ropts.to),
            Replacements::Indices(indices) => {
                replace_nth_literal(orig, &ropts.from, &ropts.to, indices)
            }
        },
        RenameAction::Prefix(text) => format!("{text}{orig}"),
        RenameAction::Suffix(text) => format!("{orig}{text}"),
        RenameAction::Renumber(ropts) => replace_nth_number(orig, ropts.index, ropts.by)?,
        RenameAction::Extension(text) => replace_extension(orig, text),
    };

    Ok(ret)
}

fn replace_extension(orig: &str, ext: &str) -> String {
    let mut new = Utf8PathBuf::from(orig);
    new.set_extension(ext);
    new.to_string()
}

fn replace_nth_number(orig: &str, index: usize, by: i64) -> anyhow::Result<String> {
    // Just do the regex every time. It won't matter.
    let rx = Regex::new(r"\d+").context("impossible regex error")?;
    let matches: Vec<_> = rx.find_iter(orig).collect();
    let mut ret = String::new();

    if let Some(m) = matches.get(index) {
        let orig_val = m.as_str().parse::<i64>()?;
        let new_val = orig_val + by;

        ret.push_str(&orig[..m.start()]);
        ret.push_str(&new_val.to_string());
        ret.push_str(&orig[m.end()..]);

        Ok(ret)
    } else {
        Ok(orig.to_owned())
    }
}

fn replace_nth_literal(orig: &str, from: &str, to: &str, indices: &[usize]) -> String {
    let mut ret = orig.to_owned();
    let matches: Vec<(usize, (usize, &str))> = orig.match_indices(from).enumerate().collect();

    for (i, (idx, _)) in matches.iter().rev() {
        if indices.contains(i) {
            ret = format!("{}{to}{}", &ret[..*idx], &ret[idx + from.len()..]);
        }
    }

    ret
}

fn replace_nth_rx(orig: &str, from: &Regex, to: &str, indices: &[usize]) -> String {
    let mut ret = orig.to_owned();

    for index in indices.iter().rev() {
        ret = match from.captures_iter(orig).nth(*index) {
            Some(captures) if let Some(whole) = captures.get(0) => {
                let mut expanded = String::new();
                captures.expand(to, &mut expanded);
                format!("{}{expanded}{}", &ret[..whole.start()], &ret[whole.end()..])
            }
            Some(_irrelevant) => ret,
            None => ret,
        }
    }
    ret
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::types::{ReplaceLiteralOpts, ReplaceRegexOpts};

    #[test]
    fn test_replace_nth_number_no_change() {
        assert_eq!(
            "no_numbers_here".to_owned(),
            replace_nth_number("no_numbers_here", 1, 1).unwrap()
        );
    }

    #[test]
    fn test_replace_nth_number_up() {
        assert_eq!(
            "make_mine_a_99".to_owned(),
            replace_nth_number("make_mine_a_98", 0, 1).unwrap()
        );
    }

    #[test]
    fn test_replace_nth_number_down() {
        assert_eq!(
            "make_mine_a_99.flac".to_owned(),
            replace_nth_number("make_mine_a_100.flac", 0, -1).unwrap()
        );
    }

    #[test]
    fn test_replace_nth_number_no_such_index() {
        assert_eq!(
            "make_mine_a_99".to_owned(),
            replace_nth_number("make_mine_a_99", 2, 1).unwrap()
        );
    }

    #[test]
    fn test_no_change_literal() {
        assert_eq!(
            "this_name_is_fine.txt",
            new_name(
                "this_name_is_fine.txt",
                &RenameAction::ReplaceLiteral(ReplaceLiteralOpts {
                    from: "bad".into(),
                    to: "right".into(),
                    replacements: Replacements::All,
                })
            )
            .unwrap()
        );
    }

    #[test]
    fn test_no_change_regex() {
        assert_eq!(
            "this_name_is_also_fine.txt",
            new_name(
                "this_name_is_also_fine.txt",
                &RenameAction::ReplaceRegex(ReplaceRegexOpts {
                    from: Regex::new("poor").unwrap(),
                    to: "better".into(),
                    replacements: Replacements::Indices(vec![0])
                })
            )
            .unwrap()
        );
    }

    #[test]
    fn test_first_match_literal() {
        assert_eq!(
            "this_also_is_the_wrong_name.txt",
            new_name(
                "this_name_is_the_wrong_name.txt",
                &RenameAction::ReplaceLiteral(ReplaceLiteralOpts {
                    from: "name".into(),
                    to: "also".into(),
                    replacements: Replacements::Indices(vec![0])
                })
            )
            .unwrap()
        );
    }

    #[test]
    fn test_first_match_regex() {
        assert_eq!(
            "this_still_is_the_wrong_name.txt",
            new_name(
                "this_name_is_the_wrong_name.txt",
                &RenameAction::ReplaceRegex(ReplaceRegexOpts {
                    from: Regex::new("(n[^_]+)").unwrap(),
                    to: "still".into(),
                    replacements: Replacements::Indices(vec![0])
                })
            )
            .unwrap()
        );
    }

    #[test]
    fn test_change_all_regex() {
        assert_eq!(
            "this_file_is_the_wrong_file.txt",
            new_name(
                "this_name_is_the_wrong_name.txt",
                &RenameAction::ReplaceRegex(ReplaceRegexOpts {
                    from: Regex::new("n[a-z][a-z]e").unwrap(),
                    to: "file".into(),
                    replacements: Replacements::All,
                })
            )
            .unwrap()
        );
    }

    #[test]
    fn test_change_all_literal() {
        assert_eq!(
            "who-puts-dots-in-filenames",
            new_name(
                "who.puts.dots.in.filenames",
                &RenameAction::ReplaceLiteral(ReplaceLiteralOpts {
                    from: ".".into(),
                    to: "-".into(),
                    replacements: Replacements::All,
                })
            )
            .unwrap()
        );
    }

    #[test]
    fn test_change_backref_first_match() {
        assert_eq!(
            "two_words",
            new_name(
                "one_word",
                &RenameAction::ReplaceRegex(ReplaceRegexOpts {
                    from: Regex::new("one_(\\w*)").unwrap(),
                    to: "two_${1}s".into(),
                    replacements: Replacements::Indices(vec![0])
                })
            )
            .unwrap()
        );
    }

    #[test]
    fn test_change_backref_swap_words() {
        assert_eq!(
            "two_cats_and_a_dog",
            new_name(
                "two_dogs_and_a_cat",
                &RenameAction::ReplaceRegex(ReplaceRegexOpts {
                    from: Regex::new("(dog)(.*)(cat)").unwrap(),
                    to: "${3}${2}${1}".into(),
                    replacements: Replacements::Indices(vec![0])
                })
            )
            .unwrap()
        );
    }

    #[test]
    fn test_change_backref_all_matches() {
        assert_eq!(
            "nerd_nerd_nerd",
            new_name(
                "word_word_word",
                &RenameAction::ReplaceRegex(ReplaceRegexOpts {
                    from: Regex::new("wo(..)").unwrap(),
                    to: "ne${1}".into(),
                    replacements: Replacements::All,
                })
            )
            .unwrap()
        );
    }

    #[test]
    fn test_change_index_rx_second() {
        assert_eq!(
            "word_new_word",
            new_name(
                "word_word_word",
                &RenameAction::ReplaceRegex(ReplaceRegexOpts {
                    from: Regex::new("w[a-z][a-z]d").unwrap(),
                    to: "new".into(),
                    replacements: Replacements::Indices(vec![1])
                })
            )
            .unwrap()
        );
    }

    #[test]
    fn test_change_index_rx_no_such_index() {
        assert_eq!(
            "word_word_word",
            new_name(
                "word_word_word",
                &RenameAction::ReplaceRegex(ReplaceRegexOpts {
                    from: Regex::new("w[a-z][a-z]d").unwrap(),
                    to: "new".into(),
                    replacements: Replacements::Indices(vec![5])
                })
            )
            .unwrap()
        );
    }

    #[test]
    fn test_change_index_rx_first_and_last() {
        assert_eq!(
            "new_word_new",
            new_name(
                "word_word_word",
                &RenameAction::ReplaceRegex(ReplaceRegexOpts {
                    from: Regex::new("w[a-z][a-z]d").unwrap(),
                    to: "new".into(),
                    replacements: Replacements::Indices(vec![0, 2])
                })
            )
            .unwrap()
        );
    }

    #[test]
    fn test_change_index_literal_first() {
        assert_eq!(
            "word_new_word",
            new_name(
                "word_word_word",
                &RenameAction::ReplaceLiteral(ReplaceLiteralOpts {
                    from: "word".into(),
                    to: "new".into(),
                    replacements: Replacements::Indices(vec![1])
                })
            )
            .unwrap()
        );
    }

    #[test]
    fn test_change_index_literal_third() {
        assert_eq!(
            "new_word_new",
            new_name(
                "word_word_word",
                &RenameAction::ReplaceLiteral(ReplaceLiteralOpts {
                    from: "word".into(),
                    to: "new".into(),
                    replacements: Replacements::Indices(vec![0, 2])
                })
            )
            .unwrap()
        );
    }

    #[test]
    fn test_change_index_literal_no_such_index() {
        assert_eq!(
            "word_word_word",
            new_name(
                "word_word_word",
                &RenameAction::ReplaceLiteral(ReplaceLiteralOpts {
                    from: "word".into(),
                    to: "new".into(),
                    replacements: Replacements::Indices(vec![9])
                })
            )
            .unwrap()
        );
    }

    #[test]
    fn test_change_for_blank() {
        assert_eq!(
            "start.end",
            new_name(
                "start.middle.end",
                &RenameAction::ReplaceLiteral(ReplaceLiteralOpts {
                    from: "middle.".into(),
                    to: String::new(),
                    replacements: Replacements::Indices(vec![0])
                })
            )
            .unwrap()
        );
    }

    #[test]
    fn test_suffix() {
        assert_eq!(
            "changed".to_string(),
            new_name("change", &RenameAction::Suffix("d".into())).unwrap()
        );
    }

    #[test]
    fn test_prefix() {
        assert_eq!(
            "newfile".to_string(),
            new_name("file", &RenameAction::Prefix("new".into())).unwrap()
        );
    }

    #[test]
    fn test_extension_change() {
        assert_eq!(
            "file.jpg".to_string(),
            new_name("file.JPEG", &RenameAction::Extension("jpg".into())).unwrap(),
        );
    }

    #[test]
    fn test_extension_add() {
        assert_eq!(
            "file.txt".to_string(),
            new_name("file", &RenameAction::Extension("txt".into())).unwrap(),
        );
    }
}

use anyhow::Context;
use regex::Regex;

#[derive(Debug)]
pub enum Replacing {
    All,
    Indices(Vec<usize>),
    Prefix(String),
    Suffix(String),
    Renumber(usize),
}

#[derive(Debug)]
pub enum FromPattern {
    Literal(String),
    Regex(Regex),
    Renumber(i64),
}

#[derive(Debug)]
pub struct RenameOpts {
    pub from: FromPattern,
    pub to: Option<String>,
    pub replacing: Replacing,
}

pub fn prefix(orig: &str, prefix: &str) -> String {
    format!("{prefix}{orig}")
}

pub fn new_name(orig: &str, opts: &RenameOpts) -> anyhow::Result<String> {
    let ret = match (&opts.replacing, &opts.from) {
        (Replacing::Prefix(pattern), _) => format!("{pattern}{orig}"),
        (Replacing::Suffix(pattern), _) => format!("{orig}{pattern}"),
        (Replacing::All, FromPattern::Literal(from)) => {
            orig.replace(from, opts.to.as_deref().context("missing 'to'")?)
        }
        (Replacing::All, FromPattern::Regex(rx)) => rx
            .replacen(orig, 0, opts.to.as_deref().context("missing 'to'")?)
            .to_string(),
        (Replacing::Indices(indices), FromPattern::Literal(from)) => replace_nth_literal(
            orig,
            from,
            opts.to.as_deref().context("missing 'to'")?,
            indices,
        ),
        (Replacing::Indices(indices), FromPattern::Regex(rx)) => replace_nth_rx(
            orig,
            rx,
            opts.to.as_deref().context("missing 'to'")?,
            indices,
        ),
        (Replacing::Renumber(index, by), _) => replace_nth_number(orig, *index, by)?,
    };

    Ok(ret)
}

fn replace_nth_number(orig: &str, index: usize, by: &str) -> anyhow::Result<String> {
    let by = by
        .parse::<i64>()
        .with_context(|| format!("first argument must be a number. (Got {by})"))?;

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
    #[test]
    fn test_replace_nth_number() {
        assert_eq!(
            "no_numbers_here".to_owned(),
            replace_nth_number("no_numbers_here", 1, "1").unwrap()
        );

        assert_eq!(
            "make_mine_a_99".to_owned(),
            replace_nth_number("make_mine_a_98", 0, "1").unwrap()
        );

        assert_eq!(
            "make_mine_a_99.flac".to_owned(),
            replace_nth_number("make_mine_a_100.flac", 0, "-1").unwrap()
        );

        assert_eq!(
            "make_mine_a_99".to_owned(),
            replace_nth_number("make_mine_a_99", 2, "1").unwrap()
        );
    }

    #[test]
    fn test_no_change_literal() {
        assert_eq!(
            "this_name_is_fine.txt",
            new_name(
                "this_name_is_fine.txt",
                &RenameOpts {
                    from: FromPattern::Literal("bad".to_owned()),
                    to: Some("right".to_owned()),
                    replacing: Replacing::Indices(vec![1])
                }
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
                &RenameOpts {
                    from: FromPattern::Regex(Regex::new("poor").unwrap().to_owned()),
                    to: Some("better".to_owned()),
                    replacing: Replacing::Indices(vec![1])
                }
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
                &RenameOpts {
                    from: FromPattern::Literal("name".to_owned()),
                    to: Some("also".to_owned()),
                    replacing: Replacing::Indices(vec![0])
                }
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
                &RenameOpts {
                    from: FromPattern::Regex(Regex::new("(n[^_]+)").unwrap().to_owned()),
                    to: Some("still".to_owned()),
                    replacing: Replacing::Indices(vec![0])
                }
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
                &RenameOpts {
                    from: FromPattern::Regex(Regex::new("n[a-z][a-z]e").unwrap().to_owned()),
                    to: Some("file".to_owned()),
                    replacing: Replacing::All,
                }
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
                &RenameOpts {
                    from: FromPattern::Literal(".".to_owned()),
                    to: Some("-".to_owned()),
                    replacing: Replacing::All,
                }
            )
            .unwrap()
        );
    }

    #[test]
    fn test_change_backref() {
        assert_eq!(
            "two_words",
            new_name(
                "one_word",
                &RenameOpts {
                    from: FromPattern::Regex(Regex::new("one_(\\w*)").unwrap()),
                    to: Some("two_${1}s".to_owned()),
                    replacing: Replacing::Indices(vec![0])
                }
            )
            .unwrap()
        );

        assert_eq!(
            "two_cats_and_a_dog",
            new_name(
                "two_dogs_and_a_cat",
                &RenameOpts {
                    from: FromPattern::Regex(Regex::new("(dog)(.*)(cat)").unwrap()),
                    to: Some("${3}${2}${1}".to_owned()),
                    replacing: Replacing::Indices(vec![0])
                }
            )
            .unwrap()
        );

        assert_eq!(
            "nerd_nerd_nerd",
            new_name(
                "word_word_word",
                &RenameOpts {
                    from: FromPattern::Regex(Regex::new("wo(..)").unwrap()),
                    to: Some("ne${1}".to_owned()),
                    replacing: Replacing::All,
                }
            )
            .unwrap()
        );
    }

    #[test]
    fn test_change_index_rx() {
        assert_eq!(
            "word_new_word",
            new_name(
                "word_word_word",
                &RenameOpts {
                    from: FromPattern::Regex(Regex::new("w[a-z][a-z]d").unwrap().to_owned()),
                    to: Some("new".to_owned()),
                    replacing: Replacing::Indices(vec![1])
                }
            )
            .unwrap()
        );

        assert_eq!(
            "word_word_word",
            new_name(
                "word_word_word",
                &RenameOpts {
                    from: FromPattern::Regex(Regex::new("w[a-z][a-z]d").unwrap().to_owned()),
                    to: Some("new".to_owned()),
                    replacing: Replacing::Indices(vec![5])
                }
            )
            .unwrap()
        );

        assert_eq!(
            "new_word_new",
            new_name(
                "word_word_word",
                &RenameOpts {
                    from: FromPattern::Regex(Regex::new("w[a-z][a-z]d").unwrap().to_owned()),
                    to: Some("new".to_owned()),
                    replacing: Replacing::Indices(vec![0, 2])
                }
            )
            .unwrap()
        );

        assert_eq!(
            "word_word_word",
            new_name(
                "word_word_word",
                &RenameOpts {
                    from: FromPattern::Regex(Regex::new("w[a-z][a-z]d").unwrap().to_owned()),
                    to: Some("new".to_owned()),
                    replacing: Replacing::Indices(vec![9])
                }
            )
            .unwrap()
        );
    }

    #[test]
    fn test_change_index_literal() {
        assert_eq!(
            "word_new_word",
            new_name(
                "word_word_word",
                &RenameOpts {
                    from: FromPattern::Literal("word".to_owned()),
                    to: Some("new".to_owned()),
                    replacing: Replacing::Indices(vec![1])
                }
            )
            .unwrap()
        );

        assert_eq!(
            "new_word_new",
            new_name(
                "word_word_word",
                &RenameOpts {
                    from: FromPattern::Literal("word".to_owned()),
                    to: Some("new".to_owned()),
                    replacing: Replacing::Indices(vec![0, 2])
                }
            )
            .unwrap()
        );

        assert_eq!(
            "word_word_word",
            new_name(
                "word_word_word",
                &RenameOpts {
                    from: FromPattern::Literal("word".to_owned()),
                    to: Some("new".to_owned()),
                    replacing: Replacing::Indices(vec![9])
                }
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
                &RenameOpts {
                    from: FromPattern::Literal("middle.".to_owned()),
                    to: Some(String::new()),
                    replacing: Replacing::Indices(vec![0])
                }
            )
            .unwrap()
        );
    }
}

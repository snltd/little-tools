use regex::Regex;

pub fn first(from: &str, to: &str, filename: &str) -> String {
    replace(from, to, filename, 1)
}

pub fn all(from: &str, to: &str, filename: &str) -> String {
    replace(from, to, filename, 0)
}

pub fn nth(from: &str, to: &str, filename: &str, index: usize) -> String {
    if index == 0 {
        replace(from, to, filename, 1)
    } else {
        replace(
            to,
            from,
            replace(from, to, filename, index + 1).as_str(),
            index,
        )
    }
}

fn replace(from: &str, to: &str, filename: &str, count: usize) -> String {
    let rx = Regex::new(from).unwrap();

    rx.replacen(filename, count, to).to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_nth() {
        assert_eq!("no_change", nth("one", "two", "no_change", 0));
        assert_eq!("one_two_one", nth("one", "two", "one_one_one", 1));
    }

    #[test]
    fn test_first() {
        assert_eq!("no_change", first("one", "two", "no_change"));
        assert_eq!("name_two", first("one", "two", "name_one"));
        assert_eq!("two_word_word", first("word", "two", "word_word_word"));
        assert_eq!("two_words", first("one_(\\w*)", "two_${1}s", "one_word"));
        assert_eq!(
            "two_cats_and_a_dog",
            first("(dog)(.*)(cat)", "${3}${2}${1}", "two_dogs_and_a_cat")
        );
        assert_eq!(
            "nerd_word_word",
            first("wo(..)", "ne${1}", "word_word_word")
        );
    }

    #[test]
    fn test_all() {
        assert_eq!("no_change", all("one", "two", "no_change"));
        assert_eq!("name_two", all("one", "two", "name_one"));
        assert_eq!("two_two_two", all("word", "two", "word_word_word"));
        assert_eq!("nerd_nerd_nerd", all("wo(..)", "ne${1}", "word_word_word"));
    }
}

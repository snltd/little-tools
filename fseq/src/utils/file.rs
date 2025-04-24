use anyhow::anyhow;
use camino::{Utf8Path, Utf8PathBuf};

pub trait PathExt {
    fn is_tagged(&self, tag: &str) -> bool;
    fn fname_tokens(&self) -> anyhow::Result<Vec<String>>;
    fn get_number(&self) -> Option<i32>;
    fn ext_as_string(&self) -> Option<String>;
}

impl PathExt for Utf8Path {
    fn is_tagged(&self, tag: &str) -> bool {
        self.to_owned().is_tagged(tag)
    }

    fn fname_tokens(&self) -> anyhow::Result<Vec<String>> {
        self.to_owned().fname_tokens()
    }

    fn get_number(&self) -> Option<i32> {
        self.to_owned().get_number()
    }

    fn ext_as_string(&self) -> Option<String> {
        self.to_owned().ext_as_string()
    }
}

impl PathExt for Utf8PathBuf {
    fn ext_as_string(&self) -> Option<String> {
        self.extension().map(|e| e.to_owned())
    }

    // How do we decide if something is tagged? In descending order of
    // strictness:
    //
    // 1. Consider a file tagged if the tag appears as any part of its name.
    //
    // 2. Consider a file tagged if the third token from the end is the tag
    //    pattern, regardless of the rest of the name. This means
    //    we'll preserve tags when we consolidate stuff that's found its way
    //    in from a different directory. BUT, this means that if a directory
    //    has untagged AND incorrectly tagged files, we'll get duplicates in
    //    the untagged.numbers vec.
    //
    // 3. Only consider files tagged if they are tagged AND correctly named.
    //    The problem with this is that something tagged but with the wrong
    //    base name (for instance, something moved from another managed
    //    directory) will lose its tag on consolidation.
    //
    // I think (2) is probably the best approach.

    fn is_tagged(&self, tag: &str) -> bool {
        match self.fname_tokens() {
            Ok(tokens) => tokens[tokens.len() - 3] == tag,
            Err(_) => false,
        }
    }

    fn fname_tokens(&self) -> anyhow::Result<Vec<String>> {
        let basename = match self.file_name() {
            Some(name) => name.to_owned(),
            None => return Err(anyhow!("Invalid file name")),
        };

        let tokens: Vec<String> = basename.split(".").map(|s| s.to_owned()).collect();

        if tokens.len() < 3 {
            return Err(anyhow!("Filename does not contain enough information"));
        }

        Ok(tokens)
    }

    fn get_number(&self) -> Option<i32> {
        let tokens = self.fname_tokens().unwrap_or_default();
        tokens[tokens.len() - 2].parse::<i32>().ok()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use test_utils::fixture;

    #[test]
    fn test_ext_as_string() {
        assert_eq!(
            Some("txt".to_string()),
            Utf8PathBuf::from("/path/file.txt").ext_as_string()
        );
    }

    #[test]
    fn test_get_number() {
        assert_eq!(
            Some(1),
            fixture("/path/to/some.dir/some.dir.0001.jpg").get_number()
        );

        assert_eq!(
            Some(99),
            fixture("/path/to/some.dir/some.dir.tag.0099.jpg").get_number()
        );

        assert_eq!(None, fixture("/path/to/some.dir/some.dir.jpg").get_number());
    }

    #[test]
    fn test_is_tagged() {
        assert!(fixture("some.dir.tag.0001.jpg").is_tagged("tag"));
        assert!(fixture("some.dir.tag.0001.jpg").is_tagged("tag"));
        assert!(fixture("/path/to/some.dir/some.dir.tag.0001.jpg").is_tagged("tag"));
        assert!(!fixture("oo.tag.oo.123.png").is_tagged("tag"));
        assert!(!fixture("/path/to/some.dir/some.dir.0001.jpg").is_tagged("tag"));
        assert!(!fixture("/path/to/some.tag.dir/some.dir.0001.jpg").is_tagged("tag"));
        assert!(!fixture("some.dir.0001.jpg").is_tagged("tag"));
        assert!(!fixture("butagy_rabbit.jpg").is_tagged("tag"));
    }
}

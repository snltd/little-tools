use crate::utils::dir;
use anyhow::{anyhow, Context};
use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

#[derive(Debug)]
pub struct FileTokens {
    pub dir: PathBuf,
    pub stem: String,
    pub num: Option<i32>,
    pub suffix: String,
    pub tag: String,
    pub is_tagged: bool,
    pub mtime: SystemTime,
}

impl FileTokens {
    pub fn new(file: &Path, tag: &str) -> anyhow::Result<FileTokens> {
        let file = file.canonicalize()?;

        let basename = file
            .file_name()
            .context("cannot get basename")?
            .to_string_lossy()
            .into_owned();

        let dirname = file.parent().context("cannot get dirname")?.to_path_buf();

        let tokens: Vec<&str> = basename.split('.').collect();
        let token_count = tokens.len();

        if token_count < 3 {
            return Err(anyhow!("too few file tokens",));
        }

        let number = tokens[token_count - 2].parse::<i32>().ok();
        let is_tagged = tag == tokens[token_count - 3];

        let mtime = fs::metadata(&file)?.modified()?;

        let stem_token_count = if is_tagged {
            token_count - 3
        } else {
            token_count - 2
        };

        Ok(FileTokens {
            dir: dirname,
            stem: tokens[..stem_token_count].join("."),
            num: number,
            suffix: tokens[token_count - 1].to_string(),
            tag: tag.to_string(),
            is_tagged,
            mtime,
        })
    }

    pub fn make_filename_with_num(&self, num: i32) -> PathBuf {
        let mut bits = vec![self.stem.clone()];
        if self.is_tagged {
            bits.push(self.tag.clone());
        }
        bits.push(dir::pad_num(num));
        bits.push(self.suffix.clone());
        self.dir.join(bits.join("."))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use test_utils::fixture;

    // Custom PartialEq for the tests. We don't want to compare mtime, because on
    // a git checkout the real one could be anything.
    impl PartialEq for FileTokens {
        fn eq(&self, other: &Self) -> bool {
            self.dir == other.dir
                && self.stem == other.stem
                && self.num == other.num
                && self.suffix == other.suffix
                && self.is_tagged == other.is_tagged
                && self.tag == other.tag
        }
    }

    #[test]
    fn test_file_tokens() {
        assert_eq!(
            FileTokens {
                dir: fixture("some.dir"),
                num: Some(2),
                stem: "some.dir".to_string(),
                suffix: "jpg".to_string(),
                is_tagged: false,
                tag: "xxx".to_string(),
                mtime: SystemTime::now(), // we don't compare this
            },
            FileTokens::new(&fixture("some.dir/some.dir.0002.jpg"), "xxx").unwrap(),
        );

        assert_eq!(
            FileTokens {
                dir: fixture("some.dir"),
                num: Some(2),
                stem: "some.dir".to_string(),
                suffix: "jpg".to_string(),
                is_tagged: true,
                tag: "tag".to_string(),
                mtime: SystemTime::now(),
            },
            FileTokens::new(&fixture("some.dir/some.dir.tag.0002.jpg"), "tag").unwrap(),
        );

        assert_eq!(
            FileTokens {
                dir: fixture("nodot"),
                num: Some(1234),
                stem: "nodot".to_string(),
                suffix: "sfx".to_string(),
                is_tagged: false,
                tag: "xxx".to_string(),
                mtime: SystemTime::now(),
            },
            FileTokens::new(&fixture("nodot/nodot.1234.sfx"), "xxx").unwrap(),
        );

        assert!(FileTokens::new(&fixture("some.dir/random_name.jpg"), "tag").is_err());
    }
}

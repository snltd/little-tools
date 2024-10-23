use crate::utils::file::PathExt;
use crate::utils::file_tokens;
use crate::utils::types;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::{fs, io};

pub type FileTokenMapSubtype = HashMap<PathBuf, file_tokens::FileTokens>;

#[derive(Debug)]
pub struct FileTokenMap {
    pub tagged: FileTokenMapSubtype,
    pub untagged: FileTokenMapSubtype,
}

pub trait DirExt {
    fn categorise_files(&self, tag: String) -> Result<FilesInDir, io::Error>;
    fn file_token_map(&self, tag: &str) -> Result<FileTokenMap, io::Error>;
}

#[derive(Debug)]
pub struct FilesInDirSubtype {
    pub dirname: PathBuf,
    pub basename: String,
    pub rogue_files: Vec<PathBuf>,
    pub numbered_files: Vec<PathBuf>,
    pub numbers: Vec<i32>,
}

pub struct FilesInDir {
    pub tagged: FilesInDirSubtype,
    pub untagged: FilesInDirSubtype,
}

impl FilesInDirSubtype {
    fn new(dirname: PathBuf, basename: &str) -> Self {
        FilesInDirSubtype {
            dirname,
            basename: basename.to_string(),
            rogue_files: Vec::new(),
            numbered_files: Vec::new(),
            numbers: Vec::new(),
        }
    }
    // Returns a list of unused numbers which will be used to rename files.
    pub fn hole_list(&self) -> Vec<i32> {
        let mut ret: Vec<i32> = Vec::new();

        if let Some(&highest_number) = self.numbers.last() {
            for i in 1..highest_number {
                if !self.numbers.contains(&i) {
                    ret.push(i);
                }
            }
        }

        ret
    }

    fn first_slot(&self) -> i32 {
        let holes = self.hole_list();

        if holes.is_empty() {
            match self.numbers.iter().max() {
                Some(max) => max + 1,
                None => 1,
            }
        } else {
            holes[0]
        }
    }

    pub fn fname_from_stem(&self, file: &PathBuf, num: i32) -> PathBuf {
        let stem = format!("{}.{}", self.basename, pad_num(num));

        let stem = match file.ext_as_string() {
            Some(ext) => format!("{}.{}", stem, ext),
            None => stem,
        };

        let fname = PathBuf::from(&stem);
        self.dirname.join(fname)
    }
}

impl FilesInDir {
    fn new(dirname: PathBuf, dir_basename: &str, tag: &str) -> Self {
        FilesInDir {
            untagged: FilesInDirSubtype::new(dirname.clone(), dir_basename),
            tagged: FilesInDirSubtype::new(
                dirname.clone(),
                format!("{}.{}", dir_basename, tag).as_str(),
            ),
        }
    }

    pub fn flip_tag(&self, file: PathBuf, tag: &str) -> types::RenameActionsResult {
        if file.is_tagged(tag) {
            self.unset_tag(file, tag)
        } else {
            self.set_tag(file, tag)
        }
    }

    pub fn set_tag(&self, file: PathBuf, tag: &str) -> types::RenameActionsResult {
        let mut ret: types::RenameActions = Vec::new();

        if !file.is_tagged(tag) {
            let target = self.tagged.fname_from_stem(&file, self.tagged.first_slot());
            ret.push((file, target));
        }

        Ok(ret)
    }

    pub fn unset_tag(&self, file: PathBuf, tag: &str) -> types::RenameActionsResult {
        let mut ret: types::RenameActions = Vec::new();

        if file.is_tagged(tag) {
            let target = self
                .untagged
                .fname_from_stem(&file, self.untagged.first_slot());
            ret.push((file, target));
        }

        Ok(ret)
    }
}

pub fn basename<P: AsRef<Path>>(path: P) -> Result<String, io::Error> {
    let path_ref: &Path = path.as_ref();

    match path_ref.file_name() {
        Some(name) => Ok(name.to_string_lossy().into_owned()),
        None => Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Invalid directory name",
        )),
    }
}

impl DirExt for Path {
    fn categorise_files(&self, tag: String) -> Result<FilesInDir, io::Error> {
        let dir_basename = basename(self)?;
        let mut ret = FilesInDir::new(self.to_path_buf(), dir_basename.as_str(), &tag);

        for file in fs::read_dir(self)? {
            let file = file?;
            let path = file.path();

            if path.is_dir() {
                continue;
            }

            let file_basename = basename(&path)?;

            if file_basename.starts_with(dir_basename.as_str()) {
                if path.is_tagged(&tag) {
                    ret.tagged.numbered_files.push(path.to_path_buf());
                    if let Some(num) = path.get_number() {
                        ret.tagged.numbers.push(num);
                    }
                } else {
                    ret.untagged.numbered_files.push(path.to_path_buf());
                    if let Some(num) = path.get_number() {
                        ret.untagged.numbers.push(num);
                    }
                }
            } else if path.is_tagged(&tag) {
                ret.tagged.rogue_files.push(path.to_path_buf());
            } else {
                ret.untagged.rogue_files.push(path.to_path_buf());
            }
        }

        ret.tagged.rogue_files.sort();
        ret.untagged.rogue_files.sort();
        ret.tagged.numbered_files.sort();
        ret.untagged.numbered_files.sort();
        ret.tagged.numbers.sort();
        ret.untagged.numbers.sort();

        Ok(ret)
    }

    fn file_token_map(&self, tag: &str) -> Result<FileTokenMap, io::Error> {
        let mut ret = FileTokenMap {
            tagged: HashMap::new(),
            untagged: HashMap::new(),
        };

        for file in fs::read_dir(self)? {
            let file = file?;
            let path = file.path();

            if path.is_dir() {
                continue;
            }

            if let Ok(tokens) = file_tokens::FileTokens::new(&path, &tag) {
                if path.is_tagged(tag) {
                    ret.tagged.insert(path, tokens);
                } else {
                    ret.untagged.insert(path, tokens);
                }
            }
        }

        Ok(ret)
    }
}

// Given a number, return a string padded with leading zeroes.
pub fn pad_num(num: i32) -> String {
    format!("{:0>4}", num)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils::spec_helper::fixture;

    #[test]
    fn test_file_token_map() {
        let result = fixture("some.dir").file_token_map("tag").unwrap();

        assert_eq!(5, result.tagged.len());
        assert_eq!(4, result.untagged.len());

        assert!(PathBuf::from("test/no/such/dir")
            .file_token_map("tag")
            .is_err());
    }

    #[test]
    fn test_set_tag() {
        let t = fixture("some.dir")
            .categorise_files("tag".to_string())
            .unwrap();

        assert_eq!(
            vec![(
                fixture("some.dir/some.dir.0001.jpg"),
                fixture("some.dir/some.dir.tag.0001.jpg")
            )],
            t.set_tag(fixture("some.dir/some.dir.0001.jpg"), "tag",)
                .unwrap(),
        );

        assert_eq!(
            vec![(
                fixture("some.dir/some.dir.0004.jpg"),
                fixture("some.dir/some.dir.tag.0001.jpg")
            )],
            t.set_tag(fixture("some.dir/some.dir.0004.jpg"), "tag",)
                .unwrap(),
        );

        assert_eq!(
            vec![(
                fixture("some.dir/whatever.JPG"),
                fixture("some.dir/some.dir.tag.0001.JPG")
            )],
            t.set_tag(fixture("some.dir/whatever.JPG"), "tag",).unwrap(),
        );

        assert!(t
            .set_tag(fixture("some.dir/some.dir.tag.0004.jpg"), "tag",)
            .unwrap()
            .is_empty(),);
    }

    #[test]
    fn test_flip_tag() {
        let t = fixture("some.dir")
            .categorise_files("tag".to_string())
            .unwrap();

        assert_eq!(
            vec![(
                fixture("some.dir/some.dir.0001.jpg"),
                fixture("some.dir/some.dir.tag.0001.jpg")
            )],
            t.flip_tag(fixture("some.dir/some.dir.0001.jpg"), "tag",)
                .unwrap(),
        );

        assert_eq!(
            vec![(
                fixture("some.dir/some.dir.tag.0004.jpg"),
                fixture("some.dir/some.dir.0004.jpg")
            )],
            t.flip_tag(fixture("some.dir/some.dir.tag.0004.jpg"), "tag",)
                .unwrap(),
        );

        assert_eq!(
            vec![(
                fixture("some.dir/whatever.JPG"),
                fixture("some.dir/some.dir.tag.0001.JPG")
            )],
            t.flip_tag(fixture("some.dir/whatever.JPG"), "tag",)
                .unwrap(),
        );
    }

    #[test]
    fn test_unset_tag() {
        let t = fixture("some.dir")
            .categorise_files("tag".to_string())
            .unwrap();

        assert_eq!(
            vec![(
                fixture("some.dir/some.dir.tag.0004.jpg"),
                fixture("some.dir/some.dir.0004.jpg")
            )],
            t.unset_tag(fixture("some.dir/some.dir.tag.0004.jpg"), "tag",)
                .unwrap(),
        );

        assert_eq!(
            vec![(
                fixture("some.dir/whatever.tag.55.JPG"),
                fixture("some.dir/some.dir.0004.JPG")
            )],
            t.unset_tag(fixture("some.dir/whatever.tag.55.JPG"), "tag",)
                .unwrap(),
        );

        assert!(t
            .unset_tag(fixture("some.dir/some.dir.0001.jpg"), "tag",)
            .unwrap()
            .is_empty(),);
    }

    #[test]
    fn test_hole_list() {
        let t = fixture("some.dir")
            .categorise_files("tag".to_string())
            .unwrap();

        assert_eq!(vec![4], t.untagged.hole_list());
        assert_eq!(1230, t.tagged.hole_list().len());
        assert_eq!(1, t.tagged.hole_list()[0]);
        assert_eq!(5, t.tagged.hole_list()[1]);
    }

    #[test]
    fn test_fname_from_stem() {
        let t = FilesInDir::new(fixture("some.dir"), "some.dir", "tag");

        assert_eq!(
            fixture("some.dir/some.dir.0045.jpg"),
            t.untagged.fname_from_stem(&fixture("rogue.jpg"), 45,)
        );
    }

    #[test]
    fn test_categorise_files() {
        let result = fixture("some.dir")
            .categorise_files("tag".to_string())
            .unwrap();

        assert_eq!("some.dir", result.untagged.basename);
        assert_eq!("some.dir.tag", result.tagged.basename);

        assert_eq!(
            vec![
                fixture("some.dir/some.dir.0001.jpg"),
                fixture("some.dir/some.dir.0002.jpg"),
                fixture("some.dir/some.dir.0003.jpg"),
                fixture("some.dir/some.dir.0005.jpg"),
            ],
            result.untagged.numbered_files,
        );

        assert_eq!(vec![1, 2, 3, 5], result.untagged.numbers);

        assert_eq!(
            vec![
                fixture("some.dir/some.dir.tag.0002.jpg"),
                fixture("some.dir/some.dir.tag.0003.jpg"),
                fixture("some.dir/some.dir.tag.0004.jpg"),
                fixture("some.dir/some.dir.tag.1234.jpg"),
            ],
            result.tagged.numbered_files,
        );

        assert_eq!(vec![2, 3, 4, 1234], result.tagged.numbers);

        assert_eq!(
            vec![
                fixture("some.dir/other_random_name.jpg"),
                fixture("some.dir/random_name.jpg"),
            ],
            result.untagged.rogue_files,
        );

        assert_eq!(
            vec![fixture("some.dir/random_name.tag.1234.jpg")],
            result.tagged.rogue_files,
        );
    }

    #[test]
    fn test_categorise_files_2() {
        let result = fixture("some.dir")
            .categorise_files("xx".to_string())
            .unwrap();

        assert_eq!("some.dir", result.untagged.basename);
        assert_eq!("some.dir.xx", result.tagged.basename);

        assert!(result.tagged.numbered_files.is_empty());
        assert!(result.tagged.numbers.is_empty());

        assert_eq!(
            vec![
                fixture("some.dir/some.dir.0001.jpg"),
                fixture("some.dir/some.dir.0002.jpg"),
                fixture("some.dir/some.dir.0003.jpg"),
                fixture("some.dir/some.dir.0005.jpg"),
                fixture("some.dir/some.dir.tag.0002.jpg"),
                fixture("some.dir/some.dir.tag.0003.jpg"),
                fixture("some.dir/some.dir.tag.0004.jpg"),
                fixture("some.dir/some.dir.tag.1234.jpg"),
            ],
            result.untagged.numbered_files,
        );
    }

    #[test]
    fn test_pad_num() {
        assert_eq!("0001", pad_num(1));
        assert_eq!("0012", pad_num(12));
        assert_eq!("0123", pad_num(123));
        assert_eq!("1234", pad_num(1234));
    }
}

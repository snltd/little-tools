use crate::action;
use crate::new_name;
use crate::types::{GlobalOpts, RenameAction};
use anyhow::{Context, bail, ensure};
use camino::{Utf8Path, Utf8PathBuf};
use std::collections::HashSet;

#[derive(Debug, PartialEq)]
pub struct Action {
    src: Utf8PathBuf,
    dest: Utf8PathBuf,
}

#[derive(Debug, PartialEq)]
pub struct ActionList(Vec<Action>);

impl ActionList {
    /// Returns a vec of to->from tuples. If the new name is the same as the original, it does
    /// not go in the list.
    pub fn new(
        paths: Vec<Utf8PathBuf>,
        action: &RenameAction,
        opts: &GlobalOpts,
    ) -> anyhow::Result<Self> {
        let mut ret = Vec::new();

        for path in paths {
            let new_path = new_path(&path, action, opts)?;

            if path != new_path {
                ret.push(Action {
                    src: path.to_owned(),
                    dest: new_path,
                });
            }
        }

        Ok(Self(ret))
    }

    pub fn check(&self) -> anyhow::Result<()> {
        let mut seen: HashSet<&Utf8PathBuf> = HashSet::new();
        let mut err = false;

        for action in self.0.iter() {
            dbg!(&action);
            if seen.contains(&action.dest) {
                let collisions: Vec<String> = self
                    .0
                    .iter()
                    .filter_map(|a| {
                        if a.dest == action.dest {
                            Some(a.src.to_string())
                        } else {
                            None
                        }
                    })
                    .collect();
                eprintln!(
                    "ERROR: multiple files have target: {} <- {}",
                    action.dest,
                    collisions.join(", ")
                );
                err = true;
            } else {
                seen.insert(&action.dest);
            }
        }

        ensure!(!err, "ERROR: collisions in target names");

        Ok(())
    }

    pub fn rename(&self, opts: &GlobalOpts) -> anyhow::Result<()> {
        for f in self.0.iter() {
            if let Err(e) = action::rename_file(&f.src, &f.dest, opts) {
                bail!("ERROR: renaming {} -> {}: {e:#}", f.src, f.dest);
            }
        }

        Ok(())
    }
}

impl From<Vec<Action>> for ActionList {
    fn from(v: Vec<Action>) -> Self {
        Self(v)
    }
}

fn new_path(
    path: &Utf8Path,
    rename_action: &RenameAction,
    global_opts: &GlobalOpts,
) -> anyhow::Result<Utf8PathBuf> {
    let source = path
        .canonicalize_utf8()
        .with_context(|| format!("cannot canonicalize {path}"))?;

    let dir = source
        .parent()
        .with_context(|| format!("cannot get parent of {source}"))?;

    // We normally only operate on the stem of the filename. Pass that to the renamer, unless
    // the user has specified that we operate on the whole filename
    let (source_stem, source_ext) = file_parts(&source, global_opts.include_ext)?;

    let new_name = new_name::new_name(source_stem, rename_action)?;

    let new_filename = if let Some(ext) = source_ext {
        format!("{new_name}.{ext}")
    } else {
        new_name
    };

    Ok(dir.join(new_filename))
}

fn file_parts(path: &Utf8Path, include_ext: bool) -> anyhow::Result<(&str, Option<&str>)> {
    let source_name = path
        .file_name()
        .with_context(|| format!("cannot get filename of {path}"))?;

    let (source_name, ext) = if include_ext {
        (source_name, None)
    } else {
        if let Some(ext) = path.extension() {
            if let Some(stem) = path.file_stem() {
                (stem, Some(ext))
            } else {
                (source_name, None)
            }
        } else {
            (source_name, None)
        }
    };

    Ok((source_name, ext))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::types::{ReplaceLiteralOpts, Replacements};
    use snltest::tmpdir_with_files;

    #[test]
    fn test_action_list_simple_replace() {
        let (_td, tp) = tmpdir_with_files(vec!["in_file_1.txt", "in_file_2.txt"]);
        let paths = vec![tp.join("in_file_1.txt"), tp.join("in_file_2.txt")];

        let expected: ActionList = vec![
            Action {
                src: tp.join("in_file_1.txt"),
                dest: tp.join("out_file_1.txt"),
            },
            Action {
                src: tp.join("in_file_2.txt"),
                dest: tp.join("out_file_2.txt"),
            },
        ]
        .into();

        let actual = ActionList::new(
            paths,
            &RenameAction::ReplaceLiteral(ReplaceLiteralOpts {
                from: "in".into(),
                to: "out".into(),
                replacements: Replacements::All,
            }),
            &GlobalOpts::default(),
        )
        .unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_action_list_02() {
        let (_td, tp) = tmpdir_with_files(vec![
            "file1.mkv",
            "file1.recoded.mkv",
            "file2.mkv",
            "file2.recoded.mkv",
        ]);

        let paths = vec![
            tp.join("file1.mkv"),
            tp.join("file1.recoded.mkv"),
            tp.join("file2.mkv"),
            tp.join("file2.recoded.mkv"),
        ];

        let expected: ActionList = vec![
            Action {
                src: tp.join("file1.recoded.mkv"),
                dest: tp.join("file1.mkv"),
            },
            Action {
                src: tp.join("file2.recoded.mkv"),
                dest: tp.join("file2.mkv"),
            },
        ]
        .into();

        let actual = ActionList::new(
            paths,
            &RenameAction::ReplaceLiteral(ReplaceLiteralOpts {
                from: ".recoded".into(),
                to: "".into(),
                replacements: Replacements::All,
            }),
            &GlobalOpts::default(),
        )
        .unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_file_parts() {
        assert_eq!(
            ("picture", Some("jpg")),
            file_parts("picture.jpg".into(), false).unwrap()
        );

        assert_eq!(
            ("picture.jpg", None),
            file_parts("picture.jpg".into(), true).unwrap()
        );

        assert_eq!(
            ("lots.and.lots.of.dots", Some("dot")),
            file_parts("lots.and.lots.of.dots.dot".into(), false).unwrap()
        );

        assert_eq!(
            ("lots.and.lots.of.dots.dot", None),
            file_parts("lots.and.lots.of.dots.dot".into(), true).unwrap()
        );

        assert_eq!(
            (".hidden", None),
            file_parts(".hidden".into(), true).unwrap()
        );
    }

    #[test]
    fn test_check_action_list_ok() {
        let list: ActionList = vec![
            Action {
                src: "/tmp/in_file_1.txt".into(),
                dest: "/tmp/out_file_1.txt".into(),
            },
            Action {
                src: "/tmp/in_file_2.txt".into(),
                dest: "/tmp/out_file_2.txt".into(),
            },
        ]
        .into();

        assert!(list.check().is_ok());
    }

    #[test]
    fn test_check_action_list_collisions() {
        let list: ActionList = vec![
            Action {
                src: "/tmp/in_file_1.txt".into(),
                dest: "/tmp/file.txt".into(),
            },
            Action {
                src: "/tmp/in_file_2.txt".into(),
                dest: "/tmp/file.txt".into(),
            },
        ]
        .into();

        let result = list.check();

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "ERROR: collisions in target names".to_string()
        )
    }
}

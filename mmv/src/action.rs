use crate::replace::{self, RenameOpts};
use anyhow::{Context, anyhow, bail};
use camino::{Utf8Path, Utf8PathBuf};
use common::verbose;
use std::collections::HashSet;
use std::fs;

pub struct ActionOpts {
    pub clobber: bool,
    pub full_names: bool,
    pub git: bool,
    pub noop: bool,
    pub terse_output: bool,
    pub verbose: bool,
}

pub fn action_list(
    paths: Vec<Utf8PathBuf>,
    include_ext: bool,
    extension: Option<String>,
    rename_opts: &RenameOpts,
) -> anyhow::Result<Vec<(Utf8PathBuf, Utf8PathBuf)>> {
    let mut ret: Vec<(Utf8PathBuf, Utf8PathBuf)> = Vec::new();

    for path in paths {
        let new_path = new_path(&path, include_ext, extension.as_deref(), rename_opts)?;
        ret.push((path.to_owned(), new_path));
    }

    Ok(ret)
}

pub fn check_action_list(action_list: &Vec<(Utf8PathBuf, Utf8PathBuf)>) -> anyhow::Result<()> {
    let mut seen: HashSet<&Utf8PathBuf> = HashSet::new();

    for (_src, dest) in action_list {
        if seen.contains(dest) {
            let collisions: Vec<String> = action_list
                .iter()
                .filter_map(|(k, v)| if v == dest { Some(k.to_string()) } else { None })
                .collect();
            bail!(
                "multiple files have same target: {} ",
                collisions.join(", ")
            );
        } else {
            seen.insert(dest);
        }
    }

    Ok(())
}

pub fn new_path(
    path: &Utf8Path,
    include_ext: bool,
    extension: Option<&str>,
    rename_opts: &RenameOpts,
) -> anyhow::Result<Utf8PathBuf> {
    let source = path
        .canonicalize_utf8()
        .with_context(|| format!("cannot canonicalize {path}"))?;

    let dir = source
        .parent()
        .with_context(|| format!("cannot get parent of {source}"))?;

    let (source_stem, source_ext) = file_parts(&source, include_ext, extension)?;
    let new_name = replace::new_name(source_stem, rename_opts);

    let new_filename = if let Some(ext) = source_ext {
        format!("{new_name}.{ext}")
    } else {
        new_name
    };

    Ok(dir.join(new_filename))
}

fn file_parts<'a>(
    path: &'a Utf8Path,
    include_ext: bool,
    extension: Option<&'a str>,
) -> anyhow::Result<(&'a str, Option<&'a str>)> {
    let source_name = path
        .file_name()
        .with_context(|| format!("cannot get filename of {path}"))?;

    let (source_name, mut ext) = if include_ext {
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

    if let Some(extn) = extension {
        ext = Some(extn)
    }

    Ok((source_name, ext))
}

pub fn rename_file(src: &Utf8Path, target: &Utf8Path, opts: &ActionOpts) -> anyhow::Result<bool> {
    let (source_name, target_name) = if opts.full_names {
        (src.to_string(), target.to_string())
    } else {
        (
            src.file_name()
                .with_context(|| format!("cannot get file name of {src}"))?
                .into(),
            target
                .file_name()
                .with_context(|| format!("cannot get file name of {src}"))?
                .into(),
        )
    };

    if target_name == source_name {
        verbose!(opts, "{}: no change", source_name);
        Ok(false)
    } else if opts.git {
        println!("git mv {} {}", src, target);
        Ok(true)
    } else {
        if opts.terse_output {
            println!("{}", target_name);
        } else {
            verbose!(opts, "{} -> {}", source_name, target_name);
        }

        rename(src, target, opts)?;
        Ok(true)
    }
}

fn rename(src: &Utf8Path, dest: &Utf8Path, opts: &ActionOpts) -> anyhow::Result<()> {
    if dest.exists() && !opts.clobber {
        Err(anyhow!("filename collision [-c to clobber]"))
    } else if !opts.noop {
        Ok(fs::rename(src, dest)?)
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::replace::{FromPattern, Replacing};
    use snltest::tmpdir_with_files;

    #[test]
    fn test_file_parts() {
        assert_eq!(
            ("picture", Some("jpg")),
            file_parts("picture.jpg".into(), false, None).unwrap()
        );

        assert_eq!(
            ("picture", Some("png")),
            file_parts("picture.jpg".into(), false, Some("png")).unwrap()
        );

        assert_eq!(
            ("file", Some("txt")),
            file_parts("file".into(), false, Some("txt")).unwrap()
        );

        assert_eq!(
            ("picture.jpg", None),
            file_parts("picture.jpg".into(), true, None).unwrap()
        );

        assert_eq!(
            ("lots.and.lots.of.dots", Some("dot")),
            file_parts("lots.and.lots.of.dots.dot".into(), false, None).unwrap()
        );

        assert_eq!(
            ("lots.and.lots.of.dots.dot", None),
            file_parts("lots.and.lots.of.dots.dot".into(), true, None).unwrap()
        );

        assert_eq!(
            (".hidden", None),
            file_parts(".hidden".into(), true, None).unwrap()
        );
    }

    #[test]
    fn test_check_action_list_ok() {
        let list = vec![
            (
                Utf8PathBuf::from("/tmp/in_file_1.txt"),
                Utf8PathBuf::from("/tmp/out_file_1.txt"),
            ),
            (
                Utf8PathBuf::from("/tmp/in_file_2.txt"),
                Utf8PathBuf::from("/tmp/out_file_2.txt"),
            ),
        ];

        assert!(check_action_list(&list).is_ok());
    }

    #[test]
    fn test_check_action_list_collisions() {
        let list = vec![
            (
                Utf8PathBuf::from("/tmp/in_file_1.txt"),
                Utf8PathBuf::from("/tmp/file.txt"),
            ),
            (
                Utf8PathBuf::from("/tmp/in_file_2.txt"),
                Utf8PathBuf::from("/tmp/file.txt"),
            ),
        ];

        let result = check_action_list(&list);

        assert!(result.is_err());
        assert!(
            result.unwrap_err().to_string().contains(
                "multiple files have same target: /tmp/in_file_1.txt, /tmp/in_file_2.txt"
            )
        );
    }

    #[test]
    fn test_action_list() {
        let (_td, tp) = tmpdir_with_files(vec!["in_file_1.txt", "in_file_2.txt"]);
        let paths = vec![tp.join("in_file_1.txt"), tp.join("in_file_2.txt")];

        let expected = vec![
            (tp.join("in_file_1.txt"), tp.join("out_file_1.txt")),
            (tp.join("in_file_2.txt"), tp.join("out_file_2.txt")),
        ];

        let actual = action_list(
            paths,
            false,
            None,
            &RenameOpts {
                from: (FromPattern::Literal("in".into())),
                to: "out".into(),
                replacing: Replacing::All,
            },
        )
        .unwrap();

        assert_eq!(expected, actual);
    }
}

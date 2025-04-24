use crate::utils::common;
use crate::utils::dir::DirExt;
use crate::utils::file_tokens::FileTokens;
use crate::utils::types::{Opts, RenameActions, RenameActionsResult};
use std::collections::HashMap;
use std::io;
use std::path::{Path, PathBuf};

type RenameActionWithIndex = Option<(usize, (PathBuf, PathBuf))>;

// Re-orders a directory, preserving tagging, changing the file numbers to match
// the mtime order of the files.

pub fn run(dirlist: &Vec<String>, opts: &Opts) -> anyhow::Result<()> {
    crate::run!(dirlist, opts)
}

fn movers_for_type(files: HashMap<PathBuf, FileTokens>) -> RenameActionsResult {
    let mut mtime_vec: Vec<(PathBuf, FileTokens)> = files.into_iter().collect();
    mtime_vec.sort_by(|a, b| a.1.mtime.cmp(&b.1.mtime));
    make_move_list(find_movers(&mtime_vec))
}

// Assumes a properly consolidated directory. Files outside the naming convention
// will be left alone.
fn actions(dir: &Path, tag: &str) -> RenameActionsResult {
    let file_map = dir.file_token_map(tag)?;
    let mut untagged = movers_for_type(file_map.untagged)?;
    let tagged = movers_for_type(file_map.tagged)?;

    untagged.extend(tagged);
    Ok(untagged)
}

// This makes a naive move list. We need to work out what order to do the moves
// in.
fn find_movers(move_vec: &[(PathBuf, FileTokens)]) -> RenameActions {
    let mut ret: RenameActions = Vec::new();
    let mut expected_number = 1;

    for (path, tokens) in move_vec.iter() {
        if let Some(actual_number) = tokens.num {
            if actual_number != expected_number {
                ret.push((
                    path.to_owned(),
                    tokens.make_filename_with_num(expected_number),
                ));
            }
            expected_number += 1;
        }
    }
    ret
}

// Returns the index and the value of the tuple in the inputs vec whose first
// (source) element is idx. (The dest from a previous move.)
fn find_next_link(inputs: &RenameActions, to_find: &PathBuf) -> RenameActionWithIndex {
    inputs
        .iter()
        .enumerate()
        .find(|(_index, (from, _to))| from == to_find)
        .map(|(index, (from, to))| (index, (from.clone(), to.clone())))
}

fn tmp_name(original_name: &Path) -> Result<PathBuf, io::Error> {
    let dir = original_name
        .parent()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "cannot make temp name"))?;

    let basename = original_name
        .file_name()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "file name missing"))?;

    Ok(dir.join(format!("_{}", basename.to_string_lossy())))
}

fn make_move_list(mut input: RenameActions) -> RenameActionsResult {
    let mut ret: RenameActions = Vec::new();

    while !input.is_empty() {
        let (mut src, dest) = input.remove(0).clone();

        match find_next_link(&input, &dest) {
            Some(_) => {
                let tmpname = tmp_name(&dest)?;
                ret.push((src.clone(), tmpname.clone()));
                input.push((tmpname, dest));
            }
            None => {
                ret.push((src.clone(), dest));
            }
        }

        while let Some((index, (next_src, next_dest))) = find_next_link(&input, &src) {
            src = next_dest.clone();
            ret.push((next_src, next_dest));
            input.remove(index);
        }
    }

    Ok(ret)
}

#[cfg(test)]
mod test {
    use super::*;
    use std::time::{Duration, SystemTime};
    use test_utils::fixture;

    #[test]
    fn test_find_next_link() {
        let inputs = &vec![
            (
                fixture("age.dir/age.dir.0002.jpg"),
                fixture("age.dir/age.dir.0001.jpg"),
            ),
            (
                fixture("age.dir/age.dir.0003.jpg"),
                fixture("age.dir/age.dir.0004.jpg"),
            ),
        ];

        assert_eq!(
            None,
            find_next_link(inputs, &fixture("age.dir/age.dir.1234.jpg"))
        );

        assert_eq!(
            Some((
                0,
                (
                    fixture("age.dir/age.dir.0002.jpg"),
                    fixture("age.dir/age.dir.0001.jpg"),
                )
            )),
            find_next_link(inputs, &fixture("age.dir/age.dir.0002.jpg"))
        );
    }

    #[test]
    fn test_make_move_list() {
        // Nothing to do.
        let empty_vec: Vec<(PathBuf, PathBuf)> = vec![];
        assert_eq!(empty_vec.clone(), make_move_list(empty_vec).unwrap());

        // One move, to an empty slot.
        assert_eq!(
            vec![(
                fixture("age.dir/age.dir.0004.jpg"),
                fixture("age.dir/age.dir.0003.jpg"),
            )],
            make_move_list(vec![(
                fixture("age.dir/age.dir.0004.jpg"),
                fixture("age.dir/age.dir.0003.jpg"),
            )])
            .unwrap()
        );

        // Two to swap. This needs a temp file.
        assert_eq!(
            vec![
                (
                    fixture("age.dir/age.dir.0003.jpg"),
                    fixture("age.dir/_age.dir.0002.jpg"),
                ),
                (
                    fixture("age.dir/age.dir.0002.jpg"),
                    fixture("age.dir/age.dir.0003.jpg"),
                ),
                (
                    fixture("age.dir/_age.dir.0002.jpg"),
                    fixture("age.dir/age.dir.0002.jpg"),
                ),
            ],
            make_move_list(vec![
                (
                    fixture("age.dir/age.dir.0003.jpg"),
                    fixture("age.dir/age.dir.0002.jpg"),
                ),
                (
                    fixture("age.dir/age.dir.0002.jpg"),
                    fixture("age.dir/age.dir.0003.jpg"),
                ),
            ])
            .unwrap()
        );

        // Reverse the order of four files
        assert_eq!(
            vec![
                (
                    fixture("age.dir/age.dir.0001.jpg"),
                    fixture("age.dir/_age.dir.0004.jpg"),
                ),
                (
                    fixture("age.dir/age.dir.0002.jpg"),
                    fixture("age.dir/_age.dir.0003.jpg"),
                ),
                (
                    fixture("age.dir/age.dir.0003.jpg"),
                    fixture("age.dir/age.dir.0002.jpg"),
                ),
                (
                    fixture("age.dir/age.dir.0004.jpg"),
                    fixture("age.dir/age.dir.0001.jpg"),
                ),
                (
                    fixture("age.dir/_age.dir.0004.jpg"),
                    fixture("age.dir/age.dir.0004.jpg"),
                ),
                (
                    fixture("age.dir/_age.dir.0003.jpg"),
                    fixture("age.dir/age.dir.0003.jpg"),
                )
            ],
            make_move_list(vec![
                (
                    fixture("age.dir/age.dir.0001.jpg"),
                    fixture("age.dir/age.dir.0004.jpg"),
                ),
                (
                    fixture("age.dir/age.dir.0002.jpg"),
                    fixture("age.dir/age.dir.0003.jpg"),
                ),
                (
                    fixture("age.dir/age.dir.0003.jpg"),
                    fixture("age.dir/age.dir.0002.jpg"),
                ),
                (
                    fixture("age.dir/age.dir.0004.jpg"),
                    fixture("age.dir/age.dir.0001.jpg"),
                ),
            ])
            .unwrap()
        );
    }

    #[test]
    fn test_find_movers() {
        let now = SystemTime::now();
        assert!(find_movers(&[
            (file_token_with_time(
                &fixture("age.dir/age.dir.0001.jpg"),
                now - Duration::new(3, 0)
            )),
            (file_token_with_time(
                &fixture("age.dir/age.dir.0002.jpg"),
                now - Duration::new(2, 0)
            )),
            (file_token_with_time(
                &fixture("age.dir/age.dir.0003.jpg"),
                now - Duration::new(1, 0)
            )),
        ])
        .is_empty(),);

        assert_eq!(
            vec![
                (
                    fixture("age.dir/age.dir.0003.jpg"),
                    fixture("age.dir/age.dir.0001.jpg"),
                ),
                (
                    fixture("age.dir/age.dir.0001.jpg"),
                    fixture("age.dir/age.dir.0003.jpg"),
                ),
            ],
            find_movers(&[
                (file_token_with_time(
                    &fixture("age.dir/age.dir.0003.jpg"),
                    now - Duration::new(3, 0)
                )),
                (file_token_with_time(
                    &fixture("age.dir/age.dir.0002.jpg"),
                    now - Duration::new(2, 0)
                )),
                (file_token_with_time(
                    &fixture("age.dir/age.dir.0001.jpg"),
                    now - Duration::new(1, 0)
                )),
            ]),
        );
    }

    fn file_token_with_time(file: &Path, ts: SystemTime) -> (PathBuf, FileTokens) {
        let mut tokens = FileTokens::new(file, "tag").unwrap();
        tokens.mtime = ts;
        (file.to_owned(), tokens)
    }
}

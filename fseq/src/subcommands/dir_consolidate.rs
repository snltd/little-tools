use crate::utils::file::PathExt;
use crate::utils::types::{Opts, RenameActions, RenameActionsResult};
use crate::utils::{common, dir, dir::DirExt};
use std::path::Path;

// Consolidates a directory. If the filename numbers are non-contiguous, pull
// down the highest numbers, renaming files until all holes are filled. Tagging
// is preserved, and filenames not matching the base pattern ("rogues") are
// renamed to fit the pattern. File extension is preserved.
//
pub fn run(dirlist: &Vec<String>, opts: &Opts) -> anyhow::Result<()> {
    crate::run!(dirlist, opts)
}

fn actions(dir: &Path, tag: &str) -> RenameActionsResult {
    let files = dir.categorise_files(tag.to_owned())?;

    let mut actions = consolidate_actions_for_base(files.untagged);
    let tagged_actions = consolidate_actions_for_base(files.tagged);

    actions.extend(tagged_actions);
    Ok(actions)
}

fn consolidate_actions_for_base(files: dir::FilesInDirSubtype) -> RenameActions {
    let mut ret: RenameActions =
        Vec::with_capacity(files.numbered_files.len() + files.rogue_files.len());
    let numbered_len = files.numbered_files.len();
    let hole_list = files.hole_list();

    let min_len = std::cmp::min(numbered_len, hole_list.len());

    for i in 0..min_len {
        let hole = hole_list[i];
        let index = numbered_len - 1 - i;

        if let Some(file_num) = files.numbered_files[index].get_number() {
            if file_num > hole {
                let source = &files.numbered_files[index];
                let target = files.fname_from_stem(source, hole);
                ret.push((source.clone(), target));
            }
        }
    }

    let starting_index = numbered_len as i32 + 1;

    ret.extend(files.rogue_files.iter().enumerate().map(|(i, file)| {
        let index = starting_index + i as i32;
        let target = files.fname_from_stem(file, index);
        (file.clone(), target)
    }));

    ret
}

#[cfg(test)]
mod test {
    use super::*;
    use test_utils::fixture;

    #[test]
    fn test_consolidate_actions() {
        let needs_work = actions(&fixture("some.dir"), "tag");

        let expected: RenameActions = vec![
            (
                fixture("some.dir/some.dir.0005.jpg"),
                fixture("some.dir/some.dir.0004.jpg"),
            ),
            (
                fixture("some.dir/other_random_name.jpg"),
                fixture("some.dir/some.dir.0005.jpg"),
            ),
            (
                fixture("some.dir/random_name.jpg"),
                fixture("some.dir/some.dir.0006.jpg"),
            ),
            (
                fixture("some.dir/some.dir.tag.1234.jpg"),
                fixture("some.dir/some.dir.tag.0001.jpg"),
            ),
            (
                fixture("some.dir/random_name.tag.1234.jpg"),
                fixture("some.dir/some.dir.tag.0005.jpg"),
            ),
        ];

        assert_eq!(expected, needs_work.unwrap());

        let expected_empty: RenameActions = Vec::new();
        assert_eq!(
            expected_empty,
            actions(&fixture("sorted.dir"), "xx").unwrap()
        );
    }
}

use camino::{Utf8Path, Utf8PathBuf};

pub fn pathbuf_set(files: &[String]) -> Vec<Utf8PathBuf> {
    let mut ret = Vec::new();

    for file in files {
        match Utf8PathBuf::from(file).canonicalize_utf8() {
            Ok(path) => ret.push(path),
            Err(_) => {
                eprintln!("WARNING: {} does not exist", file);
                continue;
            }
        };
    }

    ret
}

pub fn expand_file_list(flist: &[Utf8PathBuf], recurse: bool) -> anyhow::Result<Vec<Utf8PathBuf>> {
    let mut ret: Vec<Utf8PathBuf> = Vec::new();
    let mut dirlist: Vec<Utf8PathBuf> = Vec::new();

    for f in flist {
        if f.is_file() {
            ret.push(f.clone());
        } else if f.is_dir() {
            dirlist.push(f.clone());
        }
    }

    if recurse {
        for dir in expand_dir_list(&dirlist, true) {
            if let Ok(entries) = dir.read_dir_utf8() {
                for entry in entries.filter_map(Result::ok) {
                    let path = entry.into_path();
                    if path.is_file() {
                        ret.push(path);
                    }
                }
            }
        }
    }

    Ok(ret)
}

pub fn expand_dir_list(dirlist: &[Utf8PathBuf], recurse: bool) -> Vec<Utf8PathBuf> {
    if recurse {
        dirs_under(dirlist)
    } else {
        dirlist.iter().map(Utf8PathBuf::from).collect()
    }
}

fn dirs_under(dirs: &[Utf8PathBuf]) -> Vec<Utf8PathBuf> {
    let mut ret = Vec::new();

    for dir in dirs {
        let path = Utf8Path::new(&dir);
        if path.is_dir() {
            collect_directories(path, &mut ret);
        }
    }

    ret.into_iter().collect()
}

fn collect_directories(dir: &Utf8Path, aggr: &mut Vec<Utf8PathBuf>) {
    let dir_buf = dir.to_path_buf();

    if !aggr.contains(&dir_buf) {
        aggr.push(dir_buf);
    }

    if let Ok(entries) = dir.read_dir_utf8() {
        for entry in entries.filter_map(Result::ok) {
            if entry.path().is_dir() {
                collect_directories(entry.path(), aggr);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_unordered::assert_eq_unordered;
    use std::fs;
    use tempfile::tempdir;
    use test_utils::fixture;

    #[test]
    fn test_dirs_under() {
        let temp_dir = Utf8PathBuf::from_path_buf(tempdir().unwrap().into_path()).unwrap();
        let subdir1 = temp_dir.join("subdir1");
        let subdir2 = temp_dir.join("subdir1/subdir2");
        let subdir3 = temp_dir.join("subdir3");

        fs::create_dir_all(&subdir1).unwrap();
        fs::create_dir_all(&subdir2).unwrap();
        fs::create_dir_all(&subdir3).unwrap();

        let dirs = vec![temp_dir.clone(), subdir3.clone()];
        let all_dirs = dirs_under(&dirs);

        let expected_dirs: Vec<Utf8PathBuf> = vec![temp_dir, subdir1, subdir2, subdir3]
            .into_iter()
            .collect();

        let result_dirs: Vec<_> = all_dirs.into_iter().collect();
        assert_eq_unordered!(result_dirs, expected_dirs);
    }

    #[test]
    fn test_expand_dir_list_no_recurse() {
        let result = expand_dir_list(&[fixture("dir_1"), fixture("dir_2")], false);
        let expected = vec![fixture("dir_1"), fixture("dir_2")];
        assert_eq_unordered!(expected, result);
    }
}

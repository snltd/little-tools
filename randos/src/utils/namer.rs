use camino::Utf8PathBuf;
use rand::distr::Alphanumeric;
use rand::Rng;

pub fn name_from(path: &Utf8PathBuf, seq_no: usize, scheme: &Option<String>) -> Option<String> {
    if let Some(basename) = path.file_name() {
        let ret = match scheme {
            Some(s) => match s.as_str() {
                "hash" => hash(path),
                "random" => random(path),
                "sequential" => sequential(path, seq_no),
                "expand" => expand(path),
                _ => plain(basename),
            },
            None => plain(basename),
        };
        Some(ret)
    } else {
        None
    }
}

fn plain(name: &str) -> String {
    name.to_string()
}

fn hash(path: &Utf8PathBuf) -> String {
    let extension = path.extension().unwrap_or("");
    let path_string = path.to_string();
    let mut hasher = sha1_smol::Sha1::new();
    hasher.update(path_string.as_bytes());
    format!("{}.{}", hasher.digest(), extension)
}

fn random(path: &Utf8PathBuf) -> String {
    let extension = path.extension().unwrap_or("");

    let stem: String = rand::rng()
        .sample_iter(&Alphanumeric)
        .take(32)
        .map(char::from)
        .collect();

    format!("{}.{}", stem, extension)
}

fn sequential(path: &Utf8PathBuf, seq_no: usize) -> String {
    let extension = path.extension().unwrap_or("");
    format!("{:08}.{}", seq_no, extension)
}

fn expand(path: &Utf8PathBuf) -> String {
    path.to_string()
        .replace('/', "-")
        .trim_start_matches('-')
        .to_string()
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils::spec_helper::fixture;

    #[test]
    fn test_name_from() {
        assert_eq!(
            "file_1_3.png".to_string(),
            name_from(&fixture("dir_1/file_1_3.png"), 1, &Some("plain".into())).unwrap()
        );

        let rand = name_from(&fixture("dir_1/file_1_3.png"), 1, &Some("random".into())).unwrap();
        assert!(rand.ends_with(".png"));
        assert_eq!(36, rand.len());

        let hashed = name_from(&fixture("dir_1/file_1_3.png"), 1, &Some("hash".into())).unwrap();
        assert!(hashed.ends_with(".png"));
        assert_eq!(44, hashed.len());

        assert_eq!(
            "00000015.png".to_string(),
            name_from(
                &fixture("dir_1/file_1_3.png"),
                15,
                &Some("sequential".into())
            )
            .unwrap()
        );

        let expand = name_from(&fixture("dir_1/file_1_3.png"), 1, &Some("expand".into())).unwrap();
        assert!(expand.ends_with(".png"));
        assert!(!expand.starts_with('-'));
        assert!(!expand.contains('/'));
    }
}

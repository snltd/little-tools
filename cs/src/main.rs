use lazy_static::lazy_static;
use regex::Regex;
use std::env;
use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::{fs, io};

#[derive(Debug)]
struct File {
    dirname: PathBuf,
    basename: String,
}

lazy_static! {
    static ref WSRX: Regex = Regex::new(r"\s+").unwrap();
    static ref JUNKRX: Regex = Regex::new(r"[^a-z0-9\._\-]+").unwrap();
    static ref NUMRX: Regex =
        Regex::new(r"^(?P<prefix>.*)\.(?P<num>\d+)(?P<suffix>\.\w+)?$").unwrap();
}

fn main() {
    if env::args().len() == 1 {
        eprintln!("Usage: cs <file>...");
        std::process::exit(1);
    }

    process(std::env::args_os().skip(1).collect::<Vec<OsString>>());
}

fn process(args: Vec<OsString>) {
    for arg in args {
        let path = Path::new(&arg);

        if path.exists() {
            if let Some(p) = new_path(path) {
                rename_file(path, &p);
            }
        } else {
            eprintln!("WARN: '{}' not found.", path.display());
        }
    }
}

fn rename_file(old: &Path, new: &Path) {
    println!("{} -> {}", old.display(), new.display());
    match fs::rename(old, new) {
        Ok(_) => (),
        _ => eprintln!("ERROR: could not rename"),
    }
}

fn new_path(path: &Path) -> Option<PathBuf> {
    let file = match process_file(path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("ERROR processing {}: {}", path.display(), e);
            return None;
        }
    };

    let source = &file.basename;
    let target = safe_name(source);

    if source == &target {
        return None;
    }

    let mut new_path = file.dirname.join(&target);

    while new_path.exists() {
        new_path = numbered_path(&new_path);
    }

    Some(new_path)
}

fn process_file(path: &Path) -> io::Result<File> {
    Ok(File {
        dirname: path.canonicalize()?.parent().unwrap().to_owned(),
        basename: path.file_name().unwrap().to_string_lossy().to_string(),
    })
}

fn safe_name(old_name: &str) -> String {
    let mut ret = WSRX
        .replace_all(old_name.trim(), "_")
        .replace("_-_", "-")
        .to_lowercase();
    ret = JUNKRX.replace_all(&ret, "").to_string();

    if ret.starts_with('.') {
        ret = ret.replacen('.', "_", 1);
    }

    if ret.is_empty() {
        ret = "untranslatable".to_string();
    }

    ret
}

fn numbered_path(path: &Path) -> PathBuf {
    match process_file(path) {
        Ok(f) => f.dirname.join(uniquely_number(&f.basename)),
        _ => panic!(), // how can this happen?
    }
}

fn uniquely_number(old_name: &str) -> String {
    match NUMRX.captures(old_name) {
        Some(b) => {
            let num: i32 = match b.name("num").unwrap().as_str().parse::<i32>() {
                Ok(n) => n + 1,
                _ => 1,
            };

            let suffix = match b.name("suffix") {
                Some(n) => n.as_str(),
                None => "",
            };

            let prefix = b.name("prefix").unwrap().as_str();

            format!("{}.{:0>3?}{}", prefix, num, suffix)
        }
        None => insert_number(old_name),
    }
}

fn insert_number(old_name: &str) -> String {
    match old_name.rsplit_once('.') {
        Some(b) => format!("{}.001.{}", b.0, b.1),
        None => format!("{}.001", old_name),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_uniquely_number() {
        assert_eq!("file.001.sfx", uniquely_number("file.sfx"));
        assert_eq!("file.002.sfx", uniquely_number("file.001.sfx"));
        assert_eq!(
            "many.dots.file.022.sfx",
            uniquely_number("many.dots.file.021.sfx")
        );
        assert_eq!("file.001", uniquely_number("file"));
        assert_eq!("file.008", uniquely_number("file.007"));
        assert_eq!("unsuffixed_file.001", uniquely_number("unsuffixed_file"));
    }

    #[test]
    fn test_safe_name() {
        assert_eq!("fine".to_string(), safe_name("fine"));
        assert_eq!("downcase".to_string(), safe_name("DoWnCaSe"));
        assert_eq!(
            "w_h_i_t_e_s_p_a_c_e".to_string(),
            safe_name(" w h i t e   s p a c e ")
        );
        assert_eq!("squashed-dashes", safe_name("Squashed - Dashes"));
        assert_eq!(
            "no_nonsense.file",
            safe_name("$$No!!! NonSense:^\"£§™.FILE")
        );
        assert_eq!("_dotfile.sfx", safe_name(".dotfile.sfx"));
        assert_eq!("untranslatable", safe_name("文字化け"));
    }
}

use anyhow::{anyhow, Context};
use camino::{Utf8Path, Utf8PathBuf};
use clap::Parser;
use std::{fs, process};
use unidecode::unidecode_char;

#[derive(Parser, Debug)]
#[clap(version, about = "Filename flattener")]
struct Cli {
    /// Be verbose
    #[clap(short, long)]
    verbose: bool,
    /// Say what would happen without doing it
    #[clap(short, long)]
    noop: bool,
    /// Overwrite existing files rather than adding a number
    #[clap(short, long)]
    clobber: bool,
    /// In the event of a filename collision, error rather than putting a number in the output filename
    #[clap(short = 'N', long)]
    nonumber: bool,
    /// Files to rename
    #[arg(required = true)]
    files: Vec<Utf8PathBuf>,
}

struct Opts {
    noop: bool,
    clobber: bool,
    nonumber: bool,
    verbose: bool,
}

fn main() {
    let cli = Cli::parse();

    let mut exit_code = 0;

    let opts = Opts {
        noop: cli.noop,
        clobber: cli.clobber,
        nonumber: cli.nonumber,
        verbose: cli.verbose,
    };

    for file in cli.files {
        if let Err(e) = process_file(&file, &opts) {
            eprintln!("ERROR on {}: {}", file, e);
            exit_code = 1;
        }
    }

    process::exit(exit_code)
}

fn process_file(path: &Utf8Path, opts: &Opts) -> anyhow::Result<bool> {
    if !path.exists() {
        return Err(anyhow!("file not found"));
    }

    let path = path.canonicalize_utf8()?;
    match new_path(&path, opts)? {
        Some(new_path) => rename_file(&path, &new_path, opts),
        None => Ok(false),
    }
}

fn new_path(path: &Utf8Path, opts: &Opts) -> anyhow::Result<Option<Utf8PathBuf>> {
    let path = path.canonicalize_utf8()?;
    let basename = path.file_name().context("could not derive basename")?;
    let dir = path.parent().context("could not derive dirname")?;
    let mut new_name = ascii_filename(basename);

    if basename == new_name {
        if opts.verbose {
            println!("{} has acceptable name", path);
        }
        return Ok(None);
    }

    let mut new_path = dir.join(&new_name);

    if new_path.exists() {
        if opts.clobber {
            if opts.verbose {
                println!("{} will be overwritten", path);
            }
            return Ok(Some(new_path));
        } else if opts.nonumber {
            return Err(anyhow!("file exists: {}", new_path));
        }
    }

    loop {
        if new_path.exists() {
            let numbered_name = numbered_filename(&new_name)?;
            new_name = numbered_name;
            new_path = dir.join(&new_name);
        } else {
            break;
        }
    }

    Ok(Some(new_path))
}

fn rename_file(old: &Utf8Path, new: &Utf8Path, opts: &Opts) -> anyhow::Result<bool> {
    if opts.verbose || opts.noop {
        println!("{} -> {}", old, new);
    }

    if opts.noop {
        Ok(false)
    } else {
        fs::rename(old, new)?;
        Ok(true)
    }
}

fn ascii_filename(file_name: &str) -> String {
    let mut ret: Vec<char> = Vec::new();
    let mut last_char = '!';

    for c in file_name.chars() {
        if c.is_ascii_alphabetic() {
            ret.push(c.to_ascii_lowercase());
            last_char = c;
        } else if c.is_alphabetic() {
            let uc = ascii_filename(unidecode_char(c));
            ret.extend(uc.chars());
        } else if c.is_numeric() || ['.', '_'].contains(&c) {
            ret.push(c);
            last_char = c;
        } else if c == '-' {
            if last_char == '_' {
                let len = ret.len();
                ret[len - 1] = c;
                last_char = '_';
            } else {
                ret.push('-');
            }
        } else if c.is_whitespace() && !ret.is_empty() && last_char != '_' {
            ret.push('_');
            last_char = '_';
        }
    }

    if ret.is_empty() {
        return "UNTRANSLATABLE".into();
    }

    if ret[0] == '.' {
        ret[0] = '_';
    }

    ret.into_iter().collect()
}

fn numbered_filename(file_name: &str) -> anyhow::Result<String> {
    let mut chunks: Vec<_> = file_name.split('.').collect();
    let mut ret_chunks = Vec::new();

    if chunks.len() == 1 {
        return Ok(format!("{}.001", file_name));
    }

    let extension = chunks.pop().context("failed to get extension")?;
    let maybe_num = chunks.pop().context("failed to get filename stem")?;
    let new_num;

    ret_chunks.push(extension);

    if let Ok(num) = maybe_num.parse::<u16>() {
        new_num = format!("{:0>3}", num + 1);
        ret_chunks.push(new_num.as_str());
    } else {
        ret_chunks.push("001");
        ret_chunks.push(maybe_num);
    }

    while let Some(chunk) = chunks.pop() {
        ret_chunks.push(chunk);
    }

    ret_chunks.reverse();
    Ok(ret_chunks.join("."))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_numbered_filename() {
        assert_eq!("file.001", numbered_filename("file").unwrap());
        assert_eq!("file.001.sfx", numbered_filename("file.sfx").unwrap());
        assert_eq!("file.002.sfx", numbered_filename("file.001.sfx").unwrap());
        assert_eq!(
            "many.dots.file.022.sfx",
            numbered_filename("many.dots.file.021.sfx").unwrap()
        );
        assert_eq!("file.001.007", numbered_filename("file.007").unwrap());
    }

    #[test]
    fn test_safe_name() {
        assert_eq!("fine".to_string(), ascii_filename("fine"));
        assert_eq!("downcase".to_string(), ascii_filename("DoWnCaSe"));
        assert_eq!(
            "w_h_i_t_e_s_p_a_c_e_".to_string(),
            ascii_filename(" w h i t e   s p a c e ")
        );
        assert_eq!("squashed-dashes", ascii_filename("Squashed - Dashes"));
        assert_eq!(
            "no_nonsense.file",
            ascii_filename("$$No!!! NonSense:^\"£§™.FILE")
        );
        assert_eq!("aeneid", ascii_filename("Æneid"));
        assert_eq!("_dotfile.sfx", ascii_filename(".dotfile.sfx"));
        assert_eq!("wen_zi_hua_ke", ascii_filename("文字化け"));
        assert_eq!("UNTRANSLATABLE", ascii_filename("$(($$$$))[[$$$$]]$"));
        assert_eq!(
            "1990-02-02-no_known_cure.mp3",
            ascii_filename("1990-02-02 - No Known Cure.mp3")
        );
    }
}

#[macro_export]
macro_rules! run {
    ($dirlist:expr, $opts:expr) => {{
        let mut errs = 0;
        let tag = $opts.tag.clone();

        for dir in $dirlist {
            let actions = actions(Path::new(dir), &tag);
            if common::take_actions(actions, &$opts).is_err() {
                errs += 1;
            }
        }

        if errs > 0 {
            Err(io::Error::new(io::ErrorKind::InvalidInput, "run err"))
        } else {
            Ok(())
        }
    }};
}

#[macro_export]
macro_rules! file_tag_action {
    ($fn_name:ident, $tag_method:ident) => {
        pub fn $fn_name(flist: &Vec<String>, opts: Opts) -> Result<(), io::Error> {
            let mut errs = 0;

            for file in flist {
                let path = match Path::new(file).canonicalize() {
                    Ok(path) => path,
                    Err(e) => {
                        eprintln!("ERROR on {}: {}", file, e);
                        errs += 1;
                        continue;
                    }
                };

                match path.parent() {
                    Some(dir) => match Path::new(dir).categorise_files(opts.tag.clone()) {
                        Ok(files) => {
                            let actions = files.$tag_method(PathBuf::from(file), &opts.tag);
                            if common::take_actions(actions, &opts).is_err() {
                                errs += 1;
                            }
                        }
                        Err(e) => {
                            eprintln!("ERROR on {}: {}", file, e);
                            errs += 1;
                        }
                    },
                    None => {
                        eprintln!("ERROR: invalid file {}", file);
                        errs += 1;
                    }
                }
            }

            if errs > 0 {
                Err(io::Error::new(io::ErrorKind::Other, "invalid input"))
            } else {
                Ok(())
            }
        }
    };
}

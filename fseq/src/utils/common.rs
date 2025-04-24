use crate::utils::types::{Opts, RenameActionsResult};
use std::fs;
use std::io;

pub fn take_actions(action_list: RenameActionsResult, opts: &Opts) -> Result<(), std::io::Error> {
    let mut errs = 0;

    match action_list {
        Ok(actions) => {
            for (src, dest) in actions.iter() {
                if opts.noop || opts.verbose {
                    println!("{} -> {}", src.display(), dest.display());
                }

                if !opts.noop {
                    if dest.exists() {
                        println!("ERROR: {} exists", dest.display());
                        errs += 1;
                    } else {
                        match fs::rename(src, dest) {
                            Ok(_) => (),
                            Err(e) => {
                                println!("ERROR: {}", e);
                                errs += 1;
                            }
                        }
                    }
                }
            }
        }
        Err(e) => {
            errs += 1;
            eprintln!("ERROR: {}", e);
        }
    }

    if errs > 0 {
        Err(io::Error::new(io::ErrorKind::Other, "action errors"))
    } else {
        Ok(())
    }
}

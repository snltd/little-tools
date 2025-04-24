use crate::utils::types::{Opts, RenameActionsResult};
use anyhow::anyhow;
use common::verbose;
use std::fs;

pub fn take_actions(action_list: RenameActionsResult, opts: &Opts) -> anyhow::Result<()> {
    let mut errs = 0;

    match action_list {
        Ok(actions) => {
            for (src, dest) in actions.iter() {
                verbose!(opts, "{} -> {}", src, dest);

                if !opts.noop {
                    if dest.exists() {
                        eprintln!("ERROR: {} exists", dest);
                        errs += 1;
                    } else {
                        match fs::rename(src, dest) {
                            Ok(_) => (),
                            Err(e) => {
                                eprintln!("ERROR: {}", e);
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
        Err(anyhow!("{} action errors", errs))
    } else {
        Ok(())
    }
}

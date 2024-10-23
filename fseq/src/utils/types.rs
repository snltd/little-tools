use std::io;
use std::path::PathBuf;

pub type RenameActions = Vec<(PathBuf, PathBuf)>;
pub type RenameActionsResult = Result<RenameActions, io::Error>;

// Used to concisely pass options collected by the CLI.
#[derive(Debug)]
pub struct Opts {
    pub verbose: bool,
    pub noop: bool,
    pub tag: String,
}

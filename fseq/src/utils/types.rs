use std::path::PathBuf;

pub type RenameActions = Vec<(PathBuf, PathBuf)>;
pub type RenameActionsResult = anyhow::Result<RenameActions>;

#[derive(Debug)]
pub struct Opts {
    pub noop: bool,
    pub tag: String,
    pub verbose: bool,
}

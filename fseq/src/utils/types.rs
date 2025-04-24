use camino::Utf8PathBuf;
use std::time::SystemTime;

pub type RenameAction = (Utf8PathBuf, Utf8PathBuf);
pub type RenameActions = Vec<RenameAction>;
pub type RenameActionsResult = anyhow::Result<RenameActions>;
pub type RenameActionWithIndex = Option<(usize, RenameAction)>;
pub type PathAndTokens = (Utf8PathBuf, FileTokens);

#[derive(Debug)]
pub struct Opts {
    pub noop: bool,
    pub tag: String,
    pub verbose: bool,
}

#[derive(Debug)]
pub struct FileTokens {
    pub dir: Utf8PathBuf,
    pub stem: String,
    pub num: Option<i32>,
    pub suffix: String,
    pub tag: String,
    pub is_tagged: bool,
    pub mtime: SystemTime,
}

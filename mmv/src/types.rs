use regex::Regex;

#[derive(Debug)]
pub enum RenameAction {
    ReplaceRegex(ReplaceRegexOpts),
    ReplaceLiteral(ReplaceLiteralOpts),
    Prefix(String),
    Suffix(String),
    Renumber(RenumberOpts),
    Extension(String),
}

#[derive(Debug)]
pub enum Replacements {
    Indices(Vec<usize>),
    All,
}

#[derive(Debug, Default)]
pub struct GlobalOpts {
    pub noop: bool,
    pub terse: bool,
    pub verbose: bool,
    pub clobber: bool,
    pub full: bool,
    pub include_ext: bool,
    pub git: bool,
}

#[derive(Debug)]
pub struct ReplaceLiteralOpts {
    pub replacements: Replacements,
    pub from: String,
    pub to: String,
}

#[derive(Debug)]
pub struct ReplaceRegexOpts {
    pub replacements: Replacements,
    pub from: Regex,
    pub to: String,
}

#[derive(Debug)]
pub struct RenumberOpts {
    pub index: usize,
    pub by: i64,
    pub zeros: Option<u8>,
}

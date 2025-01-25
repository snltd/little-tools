use regex::Regex;

pub struct FilterOpts {
    pub extensions: Option<Vec<String>>,
    pub newer: Option<u64>,
    pub older: Option<u64>,
    pub regex: Option<Regex>,
}

pub struct ActionOpts {
    pub action: String,
    pub noop: bool,
    pub relative_links: bool,
    pub verbose: bool,
}

use std::path::{Path, PathBuf, MAIN_SEPARATOR};
use regex::Regex;

extern crate dirs;

pub struct Compactor {
    threshold: usize,
    limit: usize,
    symbol: String,
    absolute: bool,
    separator: String,
}
impl Compactor {
    pub fn new() -> Self {
        Self {
            threshold: 4,
            limit: 3,
            symbol: "~".to_owned(),
            absolute: false,
            separator: MAIN_SEPARATOR.to_string(),
        }
    }
    pub fn with_threshold(mut self, threshold: usize) -> Self {
        self.threshold = threshold;
        self
    }
    pub fn with_limit(mut self, limit: usize) -> Self {
        self.limit = limit;
        self
    }
    pub fn with_symbol(mut self, symbol: String) -> Self {
        self.symbol = symbol;
        self
    }
    pub fn with_absolute(mut self, absolute: bool) -> Self {
        self.absolute = absolute;
        self
    }
    pub fn with_separator<T: ToString>(mut self, separator: T) -> Self {
        self.separator = separator.to_string();
        self
    }

    pub fn compact_path(&self, path: PathBuf) -> String {
        let mut path = path;
        if !self.absolute {
            let home = get_home();

            // If the path includes the home directory, replace it with symbol
            path = match path.strip_prefix(home) {
                Ok(p) => Path::new(&self.symbol).join(p),
                Err(_) => path,
            };
        }

        // Split the path into components, compact each fragment, and return the
        // joined path
        path
            .components()
            .map(|c| self.compact_fragment(c.as_os_str().to_str().unwrap()))
            .collect::<Vec<String>>()
            .join(&self.separator)
    }

    pub fn compact_fragment(&self, fragment: &str) -> String {
        compact_fragment(fragment, self.threshold, self.limit)
    }
}

/// Gets the home directory.
fn get_home() -> PathBuf {
    match dirs::home_dir() {
        Some(p) => p,
        None => Path::new("").to_path_buf(),
    }
}

/// Compacts a single path fragment
fn compact_fragment(frag: &str, threshold: usize, size: usize) -> String {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(?:([a-zA-Z])([A-Z0-9]))|[-_.]")
            .unwrap();
    }

    if frag.chars().next() == Some(MAIN_SEPARATOR) {
        return "".to_owned();
    }

    if frag.len() <= threshold {
        return frag.to_owned();
    }

    let f = RE.replace_all(frag, "$1 $2");
    let mut parts: Vec<&str> = f
        .split_whitespace()
        .collect();
    parts.truncate(size);

    if parts.len() == 1 && frag.len() >= size {
        return frag[..size].to_owned();
    }

    parts.iter_mut()
        .map(|p| &p[..1])
        .collect::<Vec<&str>>()
        .join("")
}

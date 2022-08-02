use std::env;
use std::path::{Path, PathBuf, MAIN_SEPARATOR};
use regex::Regex;

extern crate dirs;

#[macro_use]
extern crate lazy_static;

fn main() {
    let path = get_cwd().unwrap();

    print!("{}", compact_path(path));
}

/// Gets the current working directory, or parsed from first CLI argument if
/// provided.
fn get_cwd() -> std::io::Result<PathBuf> {
    let argv: Vec<String> = env::args().collect();
    match argv.get(1) {
        Some(p) => {
            let pb = PathBuf::from(p);
            pb.canonicalize().or(Ok(pb))
        },
        _ => env::current_dir(),
    }
}

/// Gets the home directory.
fn get_home() -> PathBuf {
    match dirs::home_dir() {
        Some(p) => p,
        None => Path::new("").to_path_buf(),
    }
}

/// Compacts a provided PathBuf and returns it as a string
fn compact_path(path: PathBuf) -> String {
    let home = get_home();

    // If the path includes the home directory, replace it with "~"
    let path = match path.strip_prefix(home) {
        Ok(p) => Path::new("~").join(p),
        Err(_) => path,
    };

    // Split the path into components, compact each fragment, and return the
    // joined path
    path
        .components()
        .map(|c| compact_fragment(c.as_os_str().to_str().unwrap()))
        .collect::<Vec<String>>()
        .join(&MAIN_SEPARATOR.to_string())
}

/// Compacts a single path fragment
fn compact_fragment(frag: &str) -> String {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(?:([a-zA-Z])([A-Z0-9]))|[-_.]")
            .unwrap();
    }

    if frag.chars().nth(0) == Some(MAIN_SEPARATOR) {
        return "".to_owned();
    }

    if frag.len() <= 4 {
        return frag.to_owned();
    }

    let f = RE.replace_all(&frag, "$1 $2");
    let mut parts: Vec<&str> = f
        .split_whitespace()
        .collect();
    parts.truncate(4);

    if parts.len() == 1 {
        return frag[..3].to_owned();
    }

    parts.iter_mut()
        .map(|p| &p[..1])
        .collect::<Vec<&str>>()
        .join("")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_path() {
        let p = Path::new("/home/example/projects/my-demo-project").to_path_buf();
        let expected = vec!["", "home", "exa", "pro", "mdp"];
        let expected = expected.join(&MAIN_SEPARATOR.to_string());
        let compacted = compact_path(p);
        assert_eq!(compacted, expected);
    }

    #[test]
    fn test_frag_kebab() {
        assert_eq!(compact_fragment("some-cool-word"), "scw");
    }

    #[test]
    fn test_frag_snake() {
        assert_eq!(compact_fragment("some_cool_word"), "scw");
    }

    #[test]
    fn test_frag_camel() {
        assert_eq!(compact_fragment("someCoolWord"), "sCW");
    }
}

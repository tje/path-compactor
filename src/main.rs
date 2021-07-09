use std::env;
use std::path::{Path, PathBuf, MAIN_SEPARATOR};
use regex::Regex;

extern crate dirs;

#[macro_use]
extern crate lazy_static;

fn main() -> std::io::Result<()> {
    let path = get_cwd()?;

    print!("{}", compact_path(path));
    Ok(())
}

/// Gets the current working directory, or parsed from first CLI argument if
/// provided.
fn get_cwd() -> std::io::Result<PathBuf> {
    let argv: Vec<String> = env::args().collect();
    let path = match argv.get(1) {
        Some(p) => {
            let pb = PathBuf::from(p);
            pb.canonicalize().or(Ok(pb))
        },
        _ => env::current_dir(),
    };
    path
}

/// Gets the home directory.
fn get_home() -> PathBuf {
    let home = match dirs::home_dir() {
        Some(p) => p,
        None => Path::new("").to_path_buf(),
    };
    home
}

/// Compacts a provided PathBuf and returns it as a string
fn compact_path(path: PathBuf) -> String {
    let home = get_home();

    let mut output = String::new();

    // If the path includes the home directory, strip it out and print "~"
    let np = match path.strip_prefix(format!("{}", home.display())) {
        Ok(p) => {
            output.push_str(&"~");
            output.push(MAIN_SEPARATOR);
            p.to_path_buf()
        },
        Err(_) => path,
    };

    // Split the path by dir separator into a collection of strings and compact
    // each fragment
    let components: Vec<String> = np
        .components()
        .map(|c| c.as_os_str().to_str().unwrap())
        .map(|c| compact_fragment(c))
        .collect();

    // Iterate over each fragment in the components and push it into output,
    // prefixed with dir separator
    let mut first: bool = false;
    let mut it = components.iter();
    while let Some(frag) = it.next() {
        if first {
            output.push(MAIN_SEPARATOR);
        }
        first = true;
        output.push_str(&frag);
    }
    output
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

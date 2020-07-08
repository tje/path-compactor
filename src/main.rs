use std::env;
use std::path::{Path, PathBuf, MAIN_SEPARATOR};
use regex::Regex;

extern crate dirs;

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

    let re = Regex::new(r"(?:([a-zA-Z])([A-Z0-9]))|[-_.]").unwrap();

    // Split the path by dir separator into a collection of strings
    let components: Vec<&str> = np
        .components()
        .map(|c| c.as_os_str().to_str().unwrap())
        .collect();

    // Iterate over each fragment in the components and start printing them
    let mut first: bool = false;
    for frag in components.iter() {
        if first {
            output.push(MAIN_SEPARATOR);
        }
        first = true;

        if frag.chars().nth(0) == Some(MAIN_SEPARATOR) {
            continue;
        }

        // Just print the full fragment if it's already short
        if frag.len() <= 4 {
            output.push_str(frag);
            continue;
        }

        // Split up the fragment into lexical parts
        let f = re.replace_all(&frag, "$1 $2");
        let mut parts: Vec<&str> = f
            .split_whitespace()
            .collect();

        parts.truncate(4);

        // If there's only one word, just print the first 3 characters
        if parts.len() == 1 {
            output.push_str(&frag[..3]);
            continue;
        }

        // Print the first character of each part
        for part in parts {
            output.push_str(&part[..1]);
        }
    }
    output
}

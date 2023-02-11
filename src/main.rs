#![feature(main_separator_str)]

use std::env;
use std::path::PathBuf;

use crate::compactor::Compactor;

mod config;
mod compactor;

extern crate dirs;

#[macro_use]
extern crate lazy_static;

fn main() {
    let conf = config::get_config();
    let matches = conf.get_matches();

    let symbol = matches.get_one::<String>("symbol").unwrap().to_owned();
    let threshold = matches.get_one::<u16>("threshold").unwrap();
    let limit = matches.get_one::<u16>("limit").unwrap();
    let absolute = matches.get_flag("absolute");
    let mut compactor = Compactor::new()
        .with_absolute(absolute)
        .with_limit(*limit as usize)
        .with_symbol(symbol)
        .with_threshold(*threshold as usize);
    if let Some(separator) = matches.get_one::<String>("separator") {
        compactor = compactor.with_separator(separator);
    }

    let path = match matches.get_one::<String>("path") {
        Some(p) => {
            let pb = PathBuf::from(p);
            pb.canonicalize().or(Ok(pb))
        },
        None => env::current_dir(),
    }.expect("Could not figure out a path to compact");

    print!("{}", compactor.compact_path(path));
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::{Path, MAIN_SEPARATOR};

    #[test]
    fn test_path() {
        let p = Path::new("/home/example/projects/my-demo-project").to_path_buf();
        let expected = vec!["", "home", "exa", "pro", "mdp"];
        let expected = expected.join(&MAIN_SEPARATOR.to_string());
        // let compacted = compact_path(p, 4, 3);
        let compacted = Compactor::new().compact_path(p);
        assert_eq!(compacted, expected);
    }

    #[test]
    fn test_frag_kebab() {
        assert_eq!(Compactor::new().compact_fragment("some-cool-word"), "scw");
    }

    #[test]
    fn test_frag_snake() {
        assert_eq!(Compactor::new().compact_fragment("some_cool_word"), "scw");
    }

    #[test]
    fn test_frag_camel() {
        assert_eq!(Compactor::new().compact_fragment("someCoolWord"), "sCW");
    }

    #[test]
    fn test_threshold_single() {
        let input = "abcdefghijklmnopqrstuvwxyz";
        assert_eq!(
            Compactor::new()
                .with_threshold(10)
                .compact_fragment(input),
            "abc",
        );
        assert_eq!(
            Compactor::new()
                .with_threshold(100)
                .compact_fragment(input),
            input,
        );
    }

    #[test]
    fn test_threshold_words() {
        let input = "a few words here";
        assert_eq!(
            Compactor::new()
                .with_threshold(10)
                .compact_fragment(input),
                "afw",
        );
    }

    #[test]
    fn test_size_single() {
        let input = "abcdefghijklmnopqrstuvwxyz";
        assert_eq!(
            Compactor::new()
                .with_limit(1)
                .compact_fragment(input),
            "a",
        );
        assert_eq!(
            Compactor::new()
                .with_limit(5)
                .compact_fragment(input),
            "abcde",
        );
    }

    #[test]
    fn test_size_words() {
        let input = "a few words here";
        assert_eq!(
            Compactor::new()
                .with_threshold(1)
                .with_limit(2)
                .compact_fragment(input),
            "af",
        );
        assert_eq!(
            Compactor::new()
                .with_threshold(1)
                .with_limit(10)
                .compact_fragment(input),
            "afwh",
        );
    }

    #[test]
    fn test_delimiter() {
        assert_eq!(
            Compactor::new()
                .with_separator("+")
                .compact_path(PathBuf::from("one/two/three")),
            "one+two+thr",
        );
    }
}

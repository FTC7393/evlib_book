#!/usr/bin/env rust-script
//! ```cargo
//! [dependencies]
//! regex = "1"
//! walkdir = "2"
//! ```

use regex::Regex;
use std::borrow::Cow;

fn main() -> std::io::Result<()> {
    let path = std::env::args()
        .nth(1)
        .expect("provide a directory or file to convert");
    walkdir::WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|entry| !entry.metadata().unwrap().is_dir())
        .try_for_each(|target_path| {
            let target = std::fs::read_to_string(target_path.path())?;
            std::fs::write(target_path.path(), &*convert(&target))
        })
}

fn convert(target: &str) -> Cow<str> {
    Regex::new(r"\[\[([a-zA-Z0-9- ]*)\|?([a-zA-Z0-9 ]*)?\]\]")
        .unwrap()
        .replace_all(target, |capture: &regex::Captures| {
            let name = capture.get(1).unwrap().as_str();
            let location = {
                let maybe_location = capture.get(2).unwrap().as_str();
                (if maybe_location.len() == 0 {
                    name.replace(" ", "-")
                } else {
                    maybe_location.to_string()
                }) + ".md"
            };
            format!("[{name}]({location})")
        })
}

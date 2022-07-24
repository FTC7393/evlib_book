#!/usr/bin/env rust-script
//! ```cargo
//! [dependencies]
//! regex="1"
//! ```

use regex::Regex;
use std::borrow::Cow;

fn main() -> std::io::Result<()> {
    std::env::args().skip(1).try_for_each(|target_file_path| {
        let target = std::fs::read_to_string(&target_file_path)?;
        let fin = convert(&target);
        std::fs::write(&target_file_path, &*fin).map(|_| ())
    })
}

fn convert(target: &str) -> Cow<str> {
    let re = Regex::new(r"\[\[([a-zA-Z ]*)\]\]").unwrap();
    re.replace_all(target, |capture: &regex::Captures| {
        let name = capture.get(1).unwrap().as_str();
        let location = name.replace(" ", "-") + ".md";
        format!("[{name}]({location})")
    })
}

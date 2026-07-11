use anyhow::Result;
use clap::Parser;
use std::{collections::HashMap, io::BufRead, path::PathBuf};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    pub strings: Vec<String>,
    #[arg(short, long)]
    pub path: Option<PathBuf>,
}

#[must_use]
pub fn normalize(s: &str) -> String {
    s.trim()
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphabetic())
        .collect::<String>()
}

#[must_use]
pub fn count_words(words: &[String]) -> Vec<(String, u32)> {
    let mut map: HashMap<String, u32> = HashMap::new();

    for w in words {
        let w = normalize(w);
        map.entry(w).and_modify(|e| *e += 1).or_insert(1);
    }

    let counts = map.into_iter().collect::<Vec<(String, u32)>>();

    sort_results(counts)
}

/// Reads the file line by line.
///
/// # Errors
///
/// Will error if content read is not valid UTF-8
pub fn count_from_file(mut reader: impl BufRead) -> Result<Vec<(String, u32)>> {
    let mut line = String::new();
    let mut map: HashMap<String, u32> = HashMap::new();

    while reader.read_line(&mut line)? != 0 {
        let words = line
            .split_whitespace()
            .map(std::borrow::ToOwned::to_owned)
            .collect::<Vec<String>>();

        for (w, count) in count_words(&words) {
            map.entry(w).and_modify(|e| *e += count).or_insert(count);
        }
        line.clear();
    }
    let counts = map.into_iter().collect::<Vec<(String, u32)>>();
    Ok(sort_results(counts))
}

#[must_use]
pub fn sort_results(mut counts: Vec<(String, u32)>) -> Vec<(String, u32)> {
    counts.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));

    counts
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_words() {
        let words = vec![
            "hello".to_string(),
            "world".to_string(),
            "hello".to_string(),
        ];

        assert_eq!(
            count_words(&words),
            vec![("hello".to_string(), 2), ("world".to_string(), 1),]
        );
    }
}

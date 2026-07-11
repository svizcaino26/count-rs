use anyhow::{Context, Result};
use std::{fs::File, io::BufReader};

use clap::Parser;
use counter::{count_from_file, count_words, Args};

fn main() -> Result<()> {
    let args = Args::parse();

    if let Some(path) = &args.path {
        let f = File::open(path)
            .with_context(|| format!("Failed to open file `{}`", path.display()))?;
        let reader = BufReader::new(f);
        let word_count = count_from_file(reader)?;
        for (word, count) in word_count {
            println!("{word}: {count}");
        }
    } else {
        let word_count = count_words(&args.strings);
        for (word, count) in word_count {
            println!("{word}: {count}");
        }
    }

    Ok(())
}

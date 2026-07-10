use clap::Parser;
use counter::{count_words, Args};

fn main() {
    let args = Args::parse();

    let word_count = count_words(&args.strings);

    for (word, count) in word_count {
        println!("{word}: {count}");
    }
}

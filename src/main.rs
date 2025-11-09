use std::{env, fs};

mod stats;
use stats::{char_count, sort_char_counts, word_count};

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();
    let book_path = args
        .get(1)
        .map(|s| s.as_str())
        .unwrap_or("books/frankenstein.txt");

    let text = fs::read_to_string(book_path)?;
    let words = word_count(&text);
    let char_counts = char_count(&text);
    let sorted_counts = sort_char_counts(char_counts);
    print_report(book_path, words, &sorted_counts);
    Ok(())
}

fn print_report(book_path: &str, word_count: usize, sorted_counts: &[(char, u32)]) {
    println!("============ BOOKBOT ============");
    println!("Analyzing book found at {}...", book_path);
    println!("----------- Word Count ----------");
    println!("Found {} total words", word_count);
    println!("--------- Character Count -------");
    for &(ch, count) in sorted_counts {
        println!("{}: {}", ch, count);
    }
    println!("============= END ===============");
}

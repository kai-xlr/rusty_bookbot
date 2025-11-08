use std::error::Error;
use std::fs;

mod stats;
use stats::get_word_count;

fn main() -> Result<(), Box<dyn Error>> {
    let book_path = "books/frankenstein.txt";
    let text = get_book_text(book_path)?;
    let word_count = get_word_count(&text);
    println!("Found {} total words", word_count);
    Ok(())
}

fn get_book_text(path: &str) -> Result<String, std::io::Error> {
    fs::read_to_string(path)
}

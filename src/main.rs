use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let book_path = "books/frankenstein.txt";
    let text = get_book_text(book_path)?;
    println!("{}", text);
    Ok(())
}

fn get_book_text(path: &str) -> Result<String, std::io::Error> {
    fs::read_to_string(path)
}

# Bookbot

A Rust CLI application for analyzing text files from classic literature.

## Overview

Bookbot reads and processes classic literature texts stored in the `books/` directory, providing:
- Word count statistics
- Character frequency analysis (case-insensitive, alphabetic characters only)
- Results sorted by frequency (descending)

Currently analyzes:
- Frankenstein by Mary Shelley
- Moby Dick by Herman Melville
- Pride and Prejudice by Jane Austen

## Requirements

- Rust 1.91.0 or later (uses edition 2024)

## Usage

```bash
# Build and run
cargo run

# Run with optimizations
cargo run --release
```

## Development

```bash
# Check code without building
cargo check

# Format code
cargo fmt

# Run linter
cargo clippy

# Run tests
cargo test
```

## Project Structure

```
bookbot/
├── src/
│   ├── main.rs        # Main application entry point and report formatting
│   └── stats.rs       # Text analysis functions (word count, char frequency)
├── books/             # Classic literature text files
│   ├── frankenstein.txt
│   ├── mobydick.txt
│   └── prideandprejudice.txt
├── Cargo.toml         # Project manifest
└── README.md
```

## Implementation Details

### stats.rs
- `get_word_count(input: &str) -> usize`: Counts words by splitting on whitespace
- `char_count(input: &str) -> HashMap<char, u32>`: Builds a frequency map of lowercase characters
- `sort_char_counts(counts: HashMap<char, u32>) -> Vec<(char, u32)>`: Sorts characters by frequency (descending), then alphabetically

### main.rs
- Reads text file using `fs::read_to_string`
- Generates word and character statistics
- Prints formatted report to stdout

## License

This project is for educational purposes.

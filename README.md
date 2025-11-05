# Bookbot

A Rust CLI application for analyzing text files from classic literature.

## Overview

Bookbot reads and processes classic literature texts stored in the `books/` directory, including:
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
│   └── main.rs        # Main application logic
├── books/             # Classic literature text files
│   ├── frankenstein.txt
│   ├── mobydick.txt
│   └── prideandprejudice.txt
├── Cargo.toml         # Project manifest
└── README.md
```

## License

This project is for educational purposes.
# rusty_bookbot

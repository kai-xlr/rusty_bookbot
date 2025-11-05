# WARP.md

This file provides guidance to WARP (warp.dev) when working with code in this repository.

## Project Overview

Bookbot is a Rust CLI application for analyzing text files from classic literature. The project processes books stored in the `books/` directory (Frankenstein, Moby Dick, Pride and Prejudice).

## Common Commands

### Build and Run
```bash
# Build the project
cargo build

# Build with optimizations
cargo build --release

# Run the project
cargo run

# Run with release optimizations
cargo run --release
```

### Testing
```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run a specific test
cargo test test_name
```

### Development
```bash
# Check code without building
cargo check

# Format code
cargo fmt

# Run clippy linter
cargo clippy

# Run clippy with all warnings
cargo clippy -- -W clippy::all
```

## Architecture

### Project Structure

- **src/main.rs**: Entry point containing the main application logic
- **books/**: Directory containing text files of classic literature for analysis
- **Cargo.toml**: Project manifest with dependencies and metadata

### Key Patterns

- **File I/O**: Uses `std::fs::File` and `BufReader` for efficient file reading
- **Book Processing**: The application is designed to read and process large text files from the `books/` directory
- **Single Binary**: This is a simple CLI application compiled to a single binary

### Development Notes

- The project uses Rust edition 2024
- Currently has no external dependencies beyond the standard library
- Book files are expected to be in `books/*.txt` format
- The main entry point expects to process books starting with `frankenstein.txt`

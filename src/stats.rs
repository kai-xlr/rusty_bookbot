use std::collections::HashMap;

pub type CharCounts = HashMap<char, u32>;

/// Convert a character-count map into a sorted vector of tuples
pub fn sort_char_counts(counts: CharCounts) -> Vec<(char, u32)> {
    let mut list: Vec<(char, u32)> = counts.into_iter().collect();
    // Sort by count descending, then by character ascending for ties
    list.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
    list
}

pub fn word_count(input: &str) -> usize {
    input.split_whitespace().count()
}

pub fn char_count(input: &str) -> CharCounts {
    let mut counts = CharCounts::new();
    for ch in input
        .chars()
        .flat_map(|c| c.to_lowercase())
        .filter(|c| c.is_alphabetic())
    {
        *counts.entry(ch).or_insert(0) += 1;
    }
    counts
}

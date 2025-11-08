use std::collections::HashMap;

pub fn get_word_count(input: &str) -> usize {
    input.split_whitespace().count()
}

pub fn char_count(input: &str) -> HashMap<char, u32> {
    input.chars()
        .flat_map(|ch| ch.to_lowercase())
        .fold(HashMap::new(), |mut counts, ch| {
            *counts.entry(ch).or_insert(0) += 1;
            counts
        })
}

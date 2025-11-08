pub fn get_word_count(input: &str) -> usize {
    input.split_whitespace().count()
}

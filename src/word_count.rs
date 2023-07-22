#[derive(Debug)]
pub struct WordCountInput {
    pub bytes: bool,
    pub lines: bool,
    pub characters: bool,
    pub words: bool,
}

#[derive(Debug)]
pub struct WordCountOutput {
    bytes: usize,
    lines: usize,
    characters: usize,
    words: usize,
}

pub fn process_data(file_contents: &str, word_count_input: WordCountInput) -> WordCountOutput {
    let word_count_result = WordCountOutput {
        bytes: if word_count_input.bytes { file_contents.as_bytes().len() } else { 0 },
        lines: if word_count_input.lines { file_contents.lines().count() } else { 0 },
        words: if word_count_input.words { file_contents.split_whitespace().count() } else { 0 },
        characters: if word_count_input.characters { file_contents.chars().count() } else { 0 },
    };
    word_count_result
}
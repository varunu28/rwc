pub trait CustomOutput {
    fn to_custom_string(&self, word_count_input: WordCountInput, filename: &str) -> String;
}

pub trait CopyInput {
    fn clone(&self) -> WordCountInput;
}

pub struct WordCountInput {
    pub bytes: bool,
    pub lines: bool,
    pub characters: bool,
    pub words: bool,
}

pub struct WordCountOutput {
    pub bytes: usize,
    pub lines: usize,
    pub characters: usize,
    pub words: usize,
}

impl CustomOutput for WordCountOutput {
    fn to_custom_string(&self, word_count_input: WordCountInput, filename: &str) -> String {
        let mut result = String::from("");
        if word_count_input.bytes {
            result.push_str(" bytes: ");
            result.push_str(&self.bytes.to_string());
        }
        if word_count_input.lines {
            result.push_str(" lines: ");
            result.push_str(&self.lines.to_string());
        }
        if word_count_input.characters {
            result.push_str(" characters: ");
            result.push_str(&self.characters.to_string());
        }
        if word_count_input.words {
            result.push_str(" words: ");
            result.push_str(&self.words.to_string());
        }
        result.push_str(" ");
        result.push_str(filename);
        result
    }
}

impl CopyInput for WordCountInput {
    fn clone(&self) -> WordCountInput {
        WordCountInput {
            bytes: self.bytes,
            lines: self.lines,
            characters: self.characters,
            words: self.words,
        }
    }
}

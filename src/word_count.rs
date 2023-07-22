use crate::data::{WordCountInput, WordCountOutput};

pub fn process_data(file_contents: &str, word_count_input: WordCountInput) -> WordCountOutput {
    let word_count_result = WordCountOutput {
        bytes: if word_count_input.bytes {
            file_contents.as_bytes().len()
        } else {
            0
        },
        lines: if word_count_input.lines {
            file_contents.lines().count()
        } else {
            0
        },
        words: if word_count_input.words {
            file_contents.split_whitespace().count()
        } else {
            0
        },
        characters: if word_count_input.characters {
            file_contents.chars().count()
        } else {
            0
        },
    };
    word_count_result
}

#[cfg(test)]
mod tests {

    use super::process_data;
    use crate::data::{WordCountInput, WordCountOutput};

    #[test]
    fn test_process_success_all_enabled() {
        // Arrange
        let content = "how \n are \n you ";
        let word_count_input = WordCountInput {
            bytes: true,
            lines: true,
            characters: true,
            words: true,
        };
        let expected_byte_count = 16;
        let expected_line_count = 3;
        let expected_word_count = 3;
        let expected_character_count = 16;

        // Act
        let word_count_output: crate::word_count::WordCountOutput =
            process_data(content, word_count_input);

        // Assert
        verify_word_count_output(
            word_count_output,
            expected_byte_count,
            expected_line_count,
            expected_word_count,
            expected_character_count,
        );
    }

    #[test]
    fn test_process_success_all_disabled() {
        // Arrange
        let content = "how \n are \n you ";
        let word_count_input = WordCountInput {
            bytes: false,
            lines: false,
            characters: false,
            words: false,
        };
        let expected_byte_count = 0;
        let expected_line_count = 0;
        let expected_word_count = 0;
        let expected_character_count = 0;

        // Act
        let word_count_output: crate::word_count::WordCountOutput =
            process_data(content, word_count_input);

        // Assert
        verify_word_count_output(
            word_count_output,
            expected_byte_count,
            expected_line_count,
            expected_word_count,
            expected_character_count,
        );
    }

    #[test]
    fn test_process_success_partially_enabled() {
        // Arrange
        let content = "how \n are \n you ";
        let word_count_input = WordCountInput {
            bytes: true,
            lines: true,
            characters: false,
            words: false,
        };
        let expected_byte_count = 16;
        let expected_line_count = 3;
        let expected_word_count = 0;
        let expected_character_count = 0;

        // Act
        let word_count_output: crate::word_count::WordCountOutput =
            process_data(content, word_count_input);

        // Assert
        verify_word_count_output(
            word_count_output,
            expected_byte_count,
            expected_line_count,
            expected_word_count,
            expected_character_count,
        );
    }

    fn verify_word_count_output(
        word_count_output: WordCountOutput,
        expected_byte_count: usize,
        expected_line_count: usize,
        expected_word_count: usize,
        expected_character_count: usize,
    ) {
        assert_eq!(
            expected_byte_count, word_count_output.bytes,
            "mismatch in byte count. expected: {} found: {}",
            expected_byte_count, word_count_output.bytes
        );

        assert_eq!(
            expected_line_count, word_count_output.lines,
            "mismatch in line count. expected: {} found: {}",
            expected_line_count, word_count_output.lines
        );

        assert_eq!(
            expected_word_count, word_count_output.words,
            "mismatch in word count. expected: {} found: {}",
            expected_word_count, word_count_output.words
        );

        assert_eq!(
            expected_character_count, word_count_output.characters,
            "mismatch in character count. expected: {} found: {}",
            expected_character_count, word_count_output.characters
        );
    }
}

mod data;
mod file_reader;
mod word_count;

use data::{CopyInput, CustomOutput, WordCountInput};

fn main() {
    // read file contents
    let file_name = "data.txt";
    if let Some(file_contents) = file_reader::read_file(file_name) {
        let word_count_input = WordCountInput {
            bytes: true,
            lines: true,
            characters: true,
            words: true,
        };
        // process file contents
        let word_count_output = word_count::process_data(&file_contents, word_count_input.clone());
        println!(
            "{}",
            word_count_output.to_custom_string(word_count_input.clone(), file_name)
        );
    }
}

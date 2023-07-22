mod file_reader;
mod word_count;

use word_count::WordCountInput;

fn main() {
    // read file contents
    if let Some(file_contents) = file_reader::read_file("data.txt") {
        let word_count_input = WordCountInput {
            bytes: true,
            lines: true,
            characters: true,
            words: true,
        };
        // process file contents
        let word_count_output = word_count::process_data(&file_contents, word_count_input);
        println!("result: {:?}", word_count_output);
    }
}

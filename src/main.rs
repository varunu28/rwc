mod data;
mod file_reader;
mod word_count;

use clap::Parser;
use data::{CopyInput, CustomOutput, WordCountInput};

trait BuildWordCountInput {
    fn build_word_count_input(&self) -> WordCountInput;
}

#[derive(Parser)]
#[command(name = "rwc")]
#[command(author = "Varun Upadhyay <varun.u28@gmail.com>")]
#[command(version = "1.0")]
#[command(about = "Rust version of Linux wc utility")]
struct Cli {
    // file for wc operation
    #[arg(short, long, value_name = "file")]
    name: String,

    // line count enabled
    #[arg(short, long, value_name = "line")]
    line: bool,

    // word count enabled
    #[arg(short, long, value_name = "word")]
    word: bool,

    // byte count enabled
    #[arg(short, long, value_name = "byte")]
    byte: bool,

    // character count enabled
    #[arg(short, long, value_name = "character")]
    character: bool,
}

impl BuildWordCountInput for Cli {
    fn build_word_count_input(&self) -> WordCountInput {
        if !self.byte && !self.character && !self.line && !self.word {
            return WordCountInput {
                bytes: true,
                lines: true,
                characters: true,
                words: true,
            };
        }
        return WordCountInput {
            bytes: self.byte,
            lines: self.line,
            words: self.word,
            characters: self.character,
        };
    }
}

fn main() {
    let cli = Cli::parse();

    // read file contents
    let file_name = cli.name.as_str();
    if let Some(file_contents) = file_reader::read_file(file_name) {
        let word_count_input = cli.build_word_count_input();
        // process file contents
        let word_count_output = word_count::process_data(&file_contents, word_count_input.clone());
        println!(
            "{}",
            word_count_output.to_custom_string(word_count_input.clone(), file_name)
        );
    }
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Cli::command().debug_assert()
}

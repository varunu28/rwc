use std::fs::File;
use std::io::Read;

pub fn read_file(file_path: &str) -> Option<String> {
    let mut file = match File::open(file_path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("error while opening the file: {}", e);
            return None;
        }
    };

    let mut file_content = String::new();
    match file.read_to_string(&mut file_content) {
        Ok(_) => {
            println!("file read successfully");
        }
        Err(e) => {
            eprintln!("error while reading contents of the file: {}", e);
            return None
        }
    };
    
    Some(file_content)
}
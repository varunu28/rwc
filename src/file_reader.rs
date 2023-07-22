use std::fs::File;
use std::io::Read;

pub fn read_file(file_name: &str) -> Option<String> {
    let mut file = match File::open(file_name) {
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
            return None;
        }
    };

    Some(file_content)
}

#[cfg(test)]
mod tests {
    use crate::file_reader::read_file;

    #[test]
    fn test_file_read() {
        let some_content = read_file("data.txt");
        assert!(some_content.is_some());

        let content = some_content.unwrap();
        assert!(content.len() > 0);
    }

    #[test]
    fn test_file_not_found() {
        let result = read_file("file_not_found.txt");
        assert!(result.is_none());
    }
}

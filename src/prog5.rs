use std::fs;

pub fn run() {
    let greeting_file_result: Result<String, std::io::Error> = fs::read_to_string("hello.txt");

    match  greeting_file_result {
        Ok(file_content) => {
            println!("File read successfully: {:?}", file_content);
        },
        Err(err) => {
            println!("Failed to read file: {:?}", err);
        }
    }
}
use std::fs;           // For file system operations
use std::io;           // For handling input/output errors

fn main() -> io::Result<()> {
    // Read from a file and handle the result
    match read_from_file("a.txt") {
        Ok(content) => println!("File content:\n{}", content),
        Err(err) => eprintln!("Error reading file: {}", err),
    }

    Ok(())
}

// Function to read a file, returning a Result for error handling
fn read_from_file(file_path: &str) -> io::Result<String> {
    fs::read_to_string(file_path)
}

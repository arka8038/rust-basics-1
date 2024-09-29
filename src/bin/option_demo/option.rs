fn main() {
    // Call the function `find_first_a` and store the result in `index`.
    let index = find_first_a(String::from("possible"));

    // Match on the result to check if 'a' was found or not.
    match index {
        // If `Some(index)` is returned, print the index.
        Some(i) => println!("Index of the char 'a' is {}", i),
        // If `None` is returned, print that 'a' was not found.
        None => println!("'a' not found"),
    }
}

// Function that returns the index of the first occurrence of 'a' in the input string.
fn find_first_a(input: String) -> Option<usize> {
    // Iterate over the characters and their indices.
    for (index, ch) in input.chars().enumerate() {
        // If the character is 'a', return its index wrapped in `Some`.
        if ch == 'a' {
            return Some(index);
        }
    }
    // If no 'a' is found, return `None`.
    None
}

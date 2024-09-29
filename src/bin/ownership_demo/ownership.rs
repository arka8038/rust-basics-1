fn main() {
    demonstrate_move_semantics();
    demonstrate_clone();
    demonstrate_ownership_transfer();
    demonstrate_ownership_return();
}

// Demonstrates Rust's ownership rules and move semantics
fn demonstrate_move_semantics() {
    // `original_string` owns the heap-allocated string
    let original_string = String::from("hello, move semantics");

    // Ownership of the string data is moved from `original_string` to `moved_string`
    // After this point, `original_string` is no longer valid
    let moved_string = original_string;

    // Uncommenting the line below will result in a compile-time error since `original_string` was moved
    // print!("Original string: {}", original_string);
}

// Demonstrates cloning data to retain ownership while creating a separate copy
fn demonstrate_clone() {
    // `original_string` owns the heap-allocated string
    let original_string = String::from("hello, cloning");

    // `cloned_string` gets a deep copy of `original_string`, so both variables are valid
    let cloned_string = original_string.clone();

    // Both `original_string` and `cloned_string` can be used independently after the clone
    print!("Original string: {}\n", original_string);
    print!("Cloned string: {}\n", cloned_string);
}

// Demonstrates ownership transfer to a function, making the original variable invalid
fn demonstrate_ownership_transfer() {
    // `my_string` owns the heap-allocated string
    let my_string = String::from("hello, ownership transfer");

    // Ownership is transferred to `consume_string` function, and `my_string` is no longer valid
    consume_string(my_string);

    // Uncommenting the line below will result in a compile-time error since `my_string` was moved
    // println!("my_string after transfer: {}", my_string);
}

// Function that takes ownership of its argument, `input_string`, consuming it
fn consume_string(input_string: String) {
    println!("Consumed string: {}", input_string);
}

// Demonstrates returning ownership after passing a value to a function
fn demonstrate_ownership_return() {
    // `my_string` owns the heap-allocated string
    let mut my_string = String::from("hello, ownership return");

    // Ownership is transferred to `process_and_return_string`, but returned back to `my_string`
    my_string = process_and_return_string(my_string);

    // Now `my_string` is valid again after regaining ownership
    println!("my_string after regaining ownership: {}", my_string);
}

// Function that takes ownership of its argument and returns it back
fn process_and_return_string(input_string: String) -> String {
    println!("Processing string: {}", input_string);
    input_string // Returning ownership back to the caller
}

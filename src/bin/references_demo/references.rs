// Demonstration of Borrowing and References in Rust
fn main() {
    demonstrate_multiple_references();
    demonstrate_borrowing_function();
    demonstrate_single_mutable_reference();
    demonstrate_mutable_vs_immutable_reference();
}

// Example 1: Multiple Immutable References
fn demonstrate_multiple_references() {
    let original_string = String::from("hello");

    // Passing by reference, ownership is not transferred.
    let reference1 = &original_string; 
    let reference2 = &original_string;
    let reference3 = &original_string;

    // Multiple immutable references can exist simultaneously.
    // Ownership remains with the original variable, `original_string`.
    println!("original_string: {}", original_string);
    println!("reference1: {}", reference1);
    println!("reference2: {}", reference2);
    println!("reference3: {}", reference3);
}

// Example 2: Borrowing a Value in a Function
fn demonstrate_borrowing_function() {
    let my_string = String::from("hello, borrowing example");

    // Passing a reference to the function, which borrows `my_string`.
    print_borrowed_variable(&my_string);

    // Ownership remains with `my_string`, so it can still be used after borrowing.
    println!("my_string after borrowing: {}", my_string);
}

// Function to print a borrowed string
fn print_borrowed_variable(borrowed_string: &String) {
    // Function only has a reference, so it cannot modify the original string.
    println!("Borrowed string: {}", borrowed_string);
}

// Example 3: Mutable Reference (only one allowed at a time)
fn demonstrate_single_mutable_reference() {
    let mut mutable_string = String::from("hello, mutable example");

    // Mutable reference to the string (allows modification).
    let mutable_reference = &mut mutable_string;

    // Only one mutable reference is allowed in this scope.
    modify_string(mutable_reference);

    // Ownership returns to `mutable_string`, but the reference is no longer valid here.
    println!("mutable_string after modification: {}", mutable_string);
}

// Function to modify a mutable reference
fn modify_string(word: &mut String) {
    word.push_str(", world!");
    println!("Modified string: {}", word);
}

// Example 4: Immutable Reference Conflicts with Mutable Reference
fn demonstrate_mutable_vs_immutable_reference() {
    let mut conflict_string = String::from("hello, conflict example");

    // First, creating a mutable reference to the string.
    let mutable_ref = &mut conflict_string;

    // Uncommenting the next line will cause a compiler error:
    // let immutable_ref = &conflict_string;
    // Error: You cannot have an immutable reference while there is an active mutable reference.
    
    modify_string(mutable_ref);

    // Once the mutable reference is no longer in use, you can use the original variable.
    println!("conflict_string after modification: {}", conflict_string);
}

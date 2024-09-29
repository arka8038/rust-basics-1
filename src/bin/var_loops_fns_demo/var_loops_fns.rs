fn main() {
    // Check if a number is even
    let number = 2_000_000_000;
    println!("Is {} even? {}", number, is_even(number));

    // Calculate Fibonacci numbers using both recursive and iterative approaches
    let fib_index = 5;
    println!("Fibonacci (Recursive) at index {}: {}", fib_index, fibonacci_recursive(fib_index));
    println!("Fibonacci (Iterative) at index {}: {}", fib_index, fibonacci_iterative(fib_index));

    // Get the length of a string
    let envelope_name = String::from("Envelope");
    println!("Length of the string '{}' is {}", envelope_name, get_string_length(envelope_name));
}

/// Checks if a number is even.
/// 
/// # Parameters
/// - `num`: The number to check.
/// 
/// # Returns
/// `true` if the number is even, `false` otherwise.
fn is_even(num: i32) -> bool {
    num % 2 == 0
}

/// Calculates the Fibonacci number at a given index using recursion.
/// 
/// # Parameters
/// - `num`: The index in the Fibonacci sequence.
/// 
/// # Returns
/// The Fibonacci number at the specified index.
fn fibonacci_recursive(num: u32) -> u32 {
    match num {
        0 => 0,
        1 => 1,
        _ => fibonacci_recursive(num - 1) + fibonacci_recursive(num - 2),
    }
}

/// Calculates the Fibonacci number at a given index using iteration.
/// 
/// # Parameters
/// - `num`: The index in the Fibonacci sequence.
/// 
/// # Returns
/// The Fibonacci number at the specified index.
fn fibonacci_iterative(num: u32) -> u32 {
    if num == 0 {
        return 0;
    } else if num == 1 {
        return 1;
    }

    let mut first = 0; // Represents F(n-2)
    let mut second = 1; // Represents F(n-1)

    for _ in 2..=num {
        let next = first + second; // Calculate F(n)
        first = second; // Update F(n-2) to F(n-1)
        second = next; // Update F(n-1) to F(n)
    }

    second // F(n)
}

/// Gets the length of a given string.
/// 
/// # Parameters
/// - `s`: The string for which to find the length.
/// 
/// # Returns
/// The length of the string as `usize`.
fn get_string_length(s: String) -> usize {
    s.chars().count() // Count characters in the string
}

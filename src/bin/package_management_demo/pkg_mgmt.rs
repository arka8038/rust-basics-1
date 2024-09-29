// Importing Local and Utc from the `chrono` crate for handling date and time.
// Use `cargo add chrono` to add the chrono dependency in Cargo.toml.
use chrono::{Local, Utc};

fn main() {
    // Get the current local time
    let local_now = Local::now();
    println!("Current local time is: {}", local_now);

    // Get the current UTC time
    let utc_now = Utc::now();
    println!("Current UTC time is: {}", utc_now);
}


//Syntax to add external packages -> cargo add <package_name>
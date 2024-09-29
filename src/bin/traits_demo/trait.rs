// Traits in Rust -> Similar to Interfaces in Java, but more flexible
// Traits define shared behavior that types can implement.
trait Summary {
    // A method signature without a body would mean each type must implement it.
    // fn summarize(&self) -> String;

    // Providing a default implementation (similar to abstract classes in Java)
    fn summarize(&self) -> String {
        String::from("Hi There!") // Default behavior if not overridden by implementing types.
    }
}

// Struct defining a User with fields name and age
struct User {
    name: String,
    age: u32,
}

// Implementing the Summary trait for the User struct
impl Summary for User {
    // Override the default summarize() method with custom implementation for User
    fn summarize(&self) -> String {
        format!("Name: {}\nAge: {}", self.name, self.age)
    }
}

// Empty struct named Fix (acts as a type that implements Summary without custom behavior)
struct Fix;

// Fix struct using default implementation of Summary's summarize()
impl Summary for Fix {}

// Entry point of the program
fn main() {
    let user = User {
        name: String::from("Arka"),
        age: 24,
    };

    // Call summarize for User, which uses the custom implementation
    let summary = user.summarize();
    println!("User Summary: \n{}", summary); 

    let fix = Fix {}; // Using Fix type which has the default summarize() implementation

    notify(fix); // Calling notify() with Fix which uses default summarize()

    // notify1(fix); 
    // Uncommenting this would throw a compile-time error
    // because Fix does not implement the `Add` trait required by notify1()

    // notify1(user); 
    // Uncommenting this will cause a different error: the notify1() function
    // expects both Summary and Add traits to be implemented on the type.
}

// Function that accepts any type that implements the Summary trait
fn notify(item: impl Summary) {
    println!("Notification - Summary: \n{}", item.summarize());
}

// Define a custom trait named Add (for demonstration)
trait Add {}

// Implement Add trait for the User struct
impl Add for User {}

// Function with a more specific constraint: only types implementing both Summary and Add traits can be passed
fn notify1<T: Summary + Add>(item: T) {
    println!("Summary from item with Add trait: \n{}", item.summarize());
}

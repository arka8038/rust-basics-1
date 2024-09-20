# Enums in Rust

This project demonstrates the use of **enums in Rust** by calculating the area of two different shapes (`Rectangle` and `Circle`). The program showcases key Rust concepts such as enum definition, method implementation, pattern matching, and borrowing. This guide focuses on how enums can be used to represent multiple types of data and how to manipulate them in Rust.

## Overview

The code defines a `Shape` enum with two variants:
- `Rectangle(f64, f64)` - represents a rectangle with a width and height.
- `Circle(f64)` - represents a circle with a radius.

It uses methods to calculate the area of each shape and print whether the shape is a `Rectangle` or `Circle` along with its area.

## Code Breakdown

### 1. Enum Definition

```rust
enum Shape {
    Rectangle(f64, f64),
    Circle(f64),
}
```
- The `Shape` enum defines two variants: `Rectangle` and `Circle`. Each variant holds different data:
  - `Rectangle(f64, f64)` stores the width and height as two `f64` values.
  - `Circle(f64)` stores the radius as a single `f64` value.

### 2. Implementing Methods for Enums

You can implement methods for enums in Rust using the `impl` block. In this example, we define two methods for `Shape`: `area` and `shape_type`.

```rust
impl Shape {
    // The `area` method calculates and returns the area of the shape.
    fn area(&self) -> f64 {
        match self {
            Shape::Rectangle(width, height) => width * height,
            Shape::Circle(radius) => std::f64::consts::PI * radius * radius,  // π * r²
        }
    }

    // The `shape_type` method returns a string indicating the type of the shape.
    fn shape_type(&self) -> &'static str {
        match self {
            Shape::Rectangle(_, _) => "Rectangle",
            Shape::Circle(_) => "Circle",
        }
    }
}
```

- **Pattern Matching**: The `match` statement is used to differentiate between the `Rectangle` and `Circle` variants, extracting their respective data (width, height, or radius) to perform calculations.
- **Using Constants**: In the `Circle` variant, we use `std::f64::consts::PI` to get the value of π for area calculation.

### 3. Borrowing and Ownership in Functions

In Rust, we can pass references (`&Shape`) to functions to avoid transferring ownership of the enum instance. This is demonstrated in the `print_area` function:

```rust
fn print_area(shape: &Shape) {
    let area = shape.area();
    let shape_type = shape.shape_type();
    println!("The area of the {} is: {}", shape_type, area);
}
```

- `&Shape`: The function takes a reference to a `Shape` instead of the value itself, allowing the original value to be reused later without transferring ownership.
- `println!`: The `println!` macro is used to format and print the type of shape (using `shape_type()`) and its area.

### 4. Main Function

```rust
fn main() {
    let rectangle = Shape::Rectangle(10.0, 5.0);
    print_area(&rectangle);

    let circle = Shape::Circle(10.0);
    print_area(&circle);
}
```

- The `main` function creates instances of both `Rectangle` and `Circle` and passes them to `print_area` to print the type and area of each shape.
- The area for a rectangle (width 10.0, height 5.0) and a circle (radius 10.0) is calculated and displayed.

## Key Rust Concepts Covered

### 1. **Enums**
- **What they are**: Enums are types that can hold one of several distinct values, each of which can have associated data.
- **When to use**: Enums are useful when you have a type that can take on different forms, each requiring different handling (e.g., different shapes in this example).

### 2. **Pattern Matching**
- The `match` statement allows you to handle each enum variant in a way specific to its data.
  
### 3. **Method Implementation**
- Rust allows you to define methods for enums using `impl`. These methods help make enums more modular and reusable.

### 4. **Ownership and Borrowing**
- Passing references (`&Shape`) allows functions to operate on data without taking ownership. This is a critical part of Rust's memory safety model.

### 5. **Constants**
- Using constants like `std::f64::consts::PI` to avoid magic numbers improves readability and maintainability.

## How to Run the Code

1. Install [Rust](https://www.rust-lang.org/tools/install) if it's not already installed.
2. Save the code in a file called `main.rs`.
3. In the terminal, navigate to the directory containing the file and run the following commands:

```bash
rustc main.rs  # Compile the program
./main         # Execute the compiled program
```

## Example Output

```
The area of the Rectangle is: 50
The area of the Circle is: 314.1592653589793
```

## Conclusion

This example teaches the basic use of **enums in Rust** to represent different types of data, along with pattern matching to handle each enum variant. Additionally, you learn to define methods for enums and pass data by reference, making the code more efficient and adhering to Rust’s strict memory management rules.

---

This README explains the concepts surrounding **enums in Rust** and how they are applied in this simple program, making it a good learning resource for understanding one of the key features of Rust.
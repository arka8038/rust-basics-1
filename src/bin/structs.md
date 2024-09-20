## README: Conventions and Syntaxes Learned from the Rust Code Example

This `README` outlines the key Rust conventions, syntax, and best practices learned from the example code involving a `Rect` struct and its associated methods for calculating the area and perimeter of a rectangle.

### 1. **Struct Definition**
In Rust, a `struct` is used to define a custom data type that groups multiple related values. Each field within a struct has a name and a type.

**Syntax:**
```rust
struct StructName {
    field1: Type,
    field2: Type,
}
```

**Example:**
```rust
struct Rect {
    height: i32,
    width: i32,
}
```

- `Rect` is the struct name.
- `height` and `width` are fields of type `i32`.

### 2. **Implementing Methods with `impl`**
The `impl` block in Rust allows you to define methods for a struct. This is how you add behavior to your structs, such as calculating the area or perimeter of a rectangle.

**Syntax:**
```rust
impl StructName {
    fn method_name(&self) -> ReturnType {
        // Method body
    }
}
```

- `impl Rect` is used to define methods for the `Rect` struct.
- `&self` is a reference to the struct instance (similar to `this` in other languages).
- Methods are defined within the `impl` block using the `fn` keyword.

**Example:**
```rust
impl Rect {
    fn area(&self) -> i32 {
        self.width * self.height
    }
}
```

### 3. **Method Calls**
To call methods on a struct, you use dot notation.

**Syntax:**
```rust
let result = instance.method_name();
```

**Example:**
```rust
let area = rect1.area();
```
- Here, the `area` method is called on the `rect1` instance of the `Rect` struct.

### 4. **Ownership and Borrowing (`&self`)**
In Rust, method parameters such as `&self` borrow the struct instance without taking ownership. This allows the method to access the struct fields without consuming the struct itself. 

- `&self` is a shorthand for `self: &Self`, indicating that the method borrows the instance.
- This enables multiple method calls on the same instance without transferring ownership.

**Example:**
```rust
fn area(&self) -> i32 {
    self.width * self.height
}
```

### 5. **Struct Initialization**
To create an instance of a struct, use curly braces `{}` with field names and values. The order of fields during initialization doesn't need to match the order of declaration in the struct definition.

**Syntax:**
```rust
let instance = StructName {
    field1: value1,
    field2: value2,
};
```

**Example:**
```rust
let rect1 = Rect {
    width: 50,
    height: 20,
};
```
- `rect1` is an instance of `Rect` with `width` 50 and `height` 20.

### 6. **Printing to the Console (`println!`)**
Rust provides a macro called `println!` for printing to the console. It uses formatted strings similar to other languages like Python's `f-strings` or C's `printf`.

**Syntax:**
```rust
println!("Formatted string with variables: {}", variable);
```

**Example:**
```rust
println!("Area of the rectangle is {}", area);
```

- `{}` is a placeholder for the value of the variable `area`.

### 7. **Using `self` to Access Struct Fields**
Inside a method, the fields of the struct can be accessed using `self.field_name`. This refers to the fields of the current instance of the struct.

**Example:**
```rust
fn area(&self) -> i32 {
    self.width * self.height
}
```
- `self.width` and `self.height` access the fields of the instance `self`.

### 8. **No `return` Keyword Needed**
In Rust, you don't need to use the `return` keyword if the last expression in a function is the value you want to return. Rust will automatically return the value of the last expression.

**Example:**
```rust
fn area(&self) -> i32 {
    self.width * self.height
}
```
- The product `self.width * self.height` is implicitly returned.

### 9. **Code Formatting Conventions**
Rust has a set of coding conventions known as **Rustfmt**:
- **Snake Case for Variables and Functions**: Use `snake_case` for variable and function names.
- **Struct Names in Pascal Case**: Use `PascalCase` for struct names.
- **Curly Braces for Block Definitions**: Curly braces `{}` are used for blocks of code, such as method definitions and control structures.

**Example:**
```rust
fn perimeter(&self) -> i32 {
    2 * (self.width + self.height)
}
```

### 10. **Type Annotations**
Rust is a statically-typed language, which means all types must be known at compile time. Type annotations are used to declare the type of variables and function return types.

**Example:**
```rust
struct Rect {
    height: i32,
    width: i32,
}
```
- `i32` is a signed 32-bit integer type.

### Summary of Key Syntax Conventions:
1. **Struct Definition:**
   ```rust
   struct StructName {
       field: Type,
   }
   ```

2. **Method Implementation:**
   ```rust
   impl StructName {
       fn method_name(&self) -> ReturnType {
           // Method body
       }
   }
   ```

3. **Struct Instantiation:**
   ```rust
   let instance = StructName {
       field1: value1,
       field2: value2,
   };
   ```

4. **Method Call:**
   ```rust
   let result = instance.method_name();
   ```

5. **Console Printing:**
   ```rust
   println!("Text with variable: {}", variable);
   ```

By following these conventions and syntaxes, your Rust code will be more readable, efficient, and idiomatic.
// Define an enum called `Shape` with two variants: Rectangle and Circle.
enum Shape {
    Rectangle(f64, f64),
    Circle(f64),
}

// Implementing a method to calculate the area for the Shape enum
impl Shape {
    // `area` method calculates the area of the shape and returns it as f64.
    fn area(&self) -> f64 {
        match self {
            // For Rectangle, calculate area by multiplying width and height.
            Shape::Rectangle(width, height) => width * height,

            // For Circle, calculate area using the formula π * r².
            Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
        }
    }

    // This method returns a string indicating the type of shape (Rectangle or Circle).
    fn shape_type(&self) -> &'static str {
        match self {
            Shape::Rectangle(_, _) => "Rectangle",  // If it's a Rectangle, return "Rectangle".
            Shape::Circle(_) => "Circle",           // If it's a Circle, return "Circle".
        }
    }
}

fn main() {
    // Create a rectangle with width 10.0 and height 5.0.
    let rectangle = Shape::Rectangle(10.0, 5.0);
    
    // Print the area and type of the rectangle.
    print_area(&rectangle);

    // Create a circle with radius 10.0.
    let circle = Shape::Circle(10.0);

    // Print the area and type of the circle.
    print_area(&circle);
}

// This function prints the type of shape and its area.
// We pass a reference to the Shape enum (`&Shape`) to avoid transferring ownership.
fn print_area(shape: &Shape) {
    // Call the `area` method to get the area of the shape.
    let area = shape.area();

    // Call the `shape_type` method to get the type of the shape (Rectangle or Circle).
    let shape_type = shape.shape_type();

    // Print the type of shape and its area.
    println!("The area of the {} is: {}", shape_type, area);
}

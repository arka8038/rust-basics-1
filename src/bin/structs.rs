struct Rect {
    height: i32,
    width: i32,
}

impl Rect {
    // Calculate the area of the rectangle
    fn area(&self) -> i32 {
        // Can omit return keyword if the expression is in last line
        self.width * self.height
    }

    // Calculate the perimeter of the rectangle
    fn perimeter(&self) -> i32 {
        2 * (self.width + self.height)
    }
}

fn main() {
    let rect1 = Rect {
        width: 50,
        height: 20,
    };
    
    let area = rect1.area();
    println!("Area of the rectangle is {}", area);
    
    let perimeter = rect1.perimeter();
    println!("Perimeter of the rectangle is {}", perimeter);
}

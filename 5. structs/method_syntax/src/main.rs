// // The #[derive(Debug)] attribute automatically provides an implementation
// // of the Debug trait for the Rectangle struct, which allows us to print
// // instances of Rectangle using the {:?} formatter.
#[derive(Debug)]
struct Rectangle {
    width: u32,   // // Represents the width of the rectangle (an unsigned 32-bit integer)
    height: u32,  // // Represents the height of the rectangle (an unsigned 32-bit integer)
}

// // The impl (implementation) block is used to define methods associated with the `Rectangle` struct.
impl Rectangle {
    // // This method calculates the area of the rectangle.
    // // It takes an immutable reference to Self (&self), meaning it borrows the
    // // instance without taking ownership, and returns a u32 value. This is because
    // // we don want to modify/take ownership, we just want to read the data
    fn area(&self) -> u32 {
        // // Multiply the width by the height to compute the area.
        self.width * self.height
    }

    // // Here we created a method that has the sane name as one of the struct's fields. 
    // // This is because we want to demonstrate that Rust knows how to diferentiate between
    // // the two
    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    // // Create an instance of Rectangle named rect1 with specific width and height values.
    let rect1 = Rectangle {
        width: 30,  // // Set the rectangle's width to 30
        height: 50, // // Set the rectangle's height to 50
    };

    // // Calling the `width()` method since we are using the paranthesis
    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width) // Rust knows to call the field since it doesn't have parathesis
    }
    
    
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    // // Print the area of the rectangle using the println! macro.
    // // The area is obtained by calling the area() method on rect1.
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}

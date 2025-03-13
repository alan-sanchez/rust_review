// // The #[derive(Debug)] attribute automatically provides an implementation
// // of the Debug trait for the Rectangle struct, which allows us to print
// // instances of Rectangle using the {:?} formatter.
#[derive(Debug)]
struct Rectangle {
    width: u32,   // // Represents the width of the rectangle (an unsigned 32-bit integer)
    height: u32,  // // Represents the height of the rectangle (an unsigned 32-bit integer)
}

// // The impl block is used to define methods associated with the Rectangle struct.
impl Rectangle {
    // // This method calculates the area of the rectangle.
    // // It takes an immutable reference to self (&self), meaning it borrows the
    // // instance without taking ownership, and returns a u32 value.
    fn area(&self) -> u32 {
        // Multiply the width by the height to compute the area.
        self.width * self.height
    }
}

fn main() {
    // // Create an instance of Rectangle named rect1 with specific width and height values.
    let rect1 = Rectangle {
        width: 30,  // // Set the rectangle's width to 30
        height: 50, // // Set the rectangle's height to 50
    };

    // // Print the area of the rectangle using the println! macro.
    // // The area is obtained by calling the area() method on rect1.
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}

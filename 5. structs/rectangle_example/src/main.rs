fn main() {
    // //  Define the width of the rectangle
    let width1 = 30;
    // // Define the height of the rectangle
    let height1 = 50;

    // // Call the `area` function with `width1` and `height1` as arguments,
    // // then print the returned area value within a formatted string.
    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );

    // // Create tuple message type
    let rect2 = (30,50);

    // // Print values
    println!(
        "The area of the second rectangle is {} square pixels.", 
        area_2(rect2)
    );


    // // Define struct
    let rect3 = Rectangle{
        width: 30,
        height: 50,
    };

    println!(
        "The area of the third rectangle is {}",
        area_3(&rect3)
    );

    // // Define new struct with debugging method
    let rect4 = Rect{
        width: 30,
        height: 50,
    };

    println!("rect4 is {rect4:?}");
}

// // Create struct to help label data
struct Rectangle {
    width: u32,
    height: u32,
}

// // Included marco attemp to for debugging purposes
#[derive(Debug)]
struct Rect {
    width: u32,
    height: u32,
}

// // Define a function named `area` that calculates the area of a rectangle.
// // It takes two parameters:
// // - `width`: the width of the rectangle (of type u32)
// // - `height`: the height of the rectangle (of type u32)
// // The function returns a u32, which is the product of `width` and `height`.
fn area(width: u32, height: u32) -> u32 {
    // // Multiply width and height to compute the area.
    width * height
}

// // Refactoring code with Tuples
fn area_2(dimensions:(u32,u32)) -> u32 {
    // // To call the information from the tuple, you must use index values
    dimensions.0 * dimensions.1
}

// // Refactoring function with Struct argument
fn area_3(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
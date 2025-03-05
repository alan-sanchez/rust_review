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
    let rect1 = (30,50);

    // // Print values
    println!(
        "The area of the second rectantgle is {} square pixels.", 
        area_2(rect1)
    );
}

// Define a function named `area` that calculates the area of a rectangle.
// It takes two parameters:
// - `width`: the width of the rectangle (of type u32)
// - `height`: the height of the rectangle (of type u32)
// The function returns a u32, which is the product of `width` and `height`.
fn area(width: u32, height: u32) -> u32 {
    // // Multiply width and height to compute the area.
    width * height
}


// // Refactoring code with Tuples
fn area_2(dimensions:(u32,u32)) -> u32 {
    // // To call the information from the tuple, you must use index values
    dimensions.0 * dimensions.1
}
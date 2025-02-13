use std::io;

fn main() {
    // // Floating-Point Types // // 
    let x = 2.0; // f64 by default
    let y: f32 = 3.0; // f32 since it was defined by `:f32`


    // // Numeric Operations // // 
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 6 - 2;

    // multiplication
    let multiply = 3 * 2;

    // division
    let quotient = 56.6 / 32.2;
    let truncated = -5 / 2; // Results in -1

    // remainder
    let remainder = 43 % 5; // returns 3

    
    // // Boolean Type // // 
    let t = true;
    let f: bool = false; // with explicit type annotation


    // // The character type // // 
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';


    // // The tuple type // // 
    let tup: (i32, f64, u8) = (500, 6.4, 1); // int, float, unsigned int

    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {y}");

    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;


    // // The array type // // 
    let a = [1, 2, 3, 4, 5];

    let months = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
    
    // First argument states the type for all elements in the array, while the second element indicates the size of the arrray
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    // First argument state same value across the arry, while the second element indicates the size of the array
    let a = [3;5]; // returns [3,3,3,3,3]

    // accessing elements
    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];
    

    // // Invalid Array Element Access
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");

}

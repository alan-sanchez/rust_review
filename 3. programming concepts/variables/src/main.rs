// // 
// Two examples of variables and shadowing programming concepts. 
// // 

// // // Variable example
// fn main() {
//     // // Declare a mutable variable `x` and initialize it with the value 5
//     // // The `mut` keyword allows us to change the value of `x` later
//     let mut x = 5;
//     println!("The value of x is: {x}");
    
//     // // Change the value of `x` from 5 to 6
//     // // Since `x` was declared as mutable (`mut`), this reassignment is allowed
//     x = 6;
//     println!("The value of x is: {x}");
// }


// Shadowing example in Rust
fn main() {
    // Declare a variable `x` and initialize it with the value 5.
    let x = 5;

    // Shadowing: Declare a new `x` (which hides the previous `x`) 
    // and assign it the value of the old `x` plus 1. Now `x = 6`.
    let x = x + 1;

    {
        // This is a new scope (inner block).
        // Shadow `x` again, multiplying its value by 2 (6 * 2 = 12).
        let x = x * 2;

        // This `println!` statement uses the inner `x`, which is 12.
        println!("The value of x in the inner scope is: {x}");
    } // The inner `x` goes out of scope here.

    // Outside the inner scope, `x` still holds the value 6 (from the second let statement).
    println!("The value of x is: {x}");
}

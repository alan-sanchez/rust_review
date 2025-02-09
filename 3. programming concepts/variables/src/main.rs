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



// // Shadowing example
fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}


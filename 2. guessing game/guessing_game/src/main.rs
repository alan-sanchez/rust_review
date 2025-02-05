// // To obtain user input and then print the result as output, we need to bring the io 
// // input/output library into scope. The io library comes from the standard library, known as std:
use std::io;

// // The main function is the entry point into the program 
// // The `fn` syntax delcares a new function;; the parentheses, (), indicate no parameters
fn main() {

    // // println! is a macro that prints a string to the screen
    println!("Guess the number!");

    println!("Please input your guess.");

    // // `let` statement is used to create a variable. Since variables are immutable by deafult, 
    // // you need to make the variable mutabale by adding `mut` before the variable name
    // // On the right of the equal sign, the variable `guess` is bound to a function that returns a new instance of a string. 
    let mut guess = String::new();

    // '''
    // The following block of text is part of a single logical line of code. 
    // '''
    // // There is the io module (I think similar to a class) that calls the `stdin` function,
    // // which allows to handle user input
    io::stdin()
        // // `read_line` is a method on the standard input handl to get input from the user
        // // We're also passing &mut guess as the argument to read_line to tell it what string to stroe the user input in
        // // the `&` indicates that this argument is a reference to the variable guess
        .read_line(&mut guess)

        // // Will crash and give you a indicator why
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}

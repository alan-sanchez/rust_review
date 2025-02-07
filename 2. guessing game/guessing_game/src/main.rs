// // To obtain user input and then print the result as output, we need to bring the io 
// // input/output library into scope. The io library comes from the standard library, known as std
use std::io;


// // The Rng trait defines methods that random number generators implement, and this 
// // trait must be in scope for us to use those methods
use rand::Rng;

// // Calling the Odering type into scop from the standard library. The Odering type is another
// // enum and has the variants Less, Greater, and Equal.
use std::cmp::Ordering;


// // The main function is the entry point into the program 
// // The `fn` syntax delcares a new function;; the parentheses, (), indicate no parameters
fn main() {

    // // println! is a macro that prints a string to the screen
    println!("Guess the number!");

    // // In the first line, we call the rand::rng function that gives us the particular random 
    // // number generator we’re going to use
    // // 
    // // Then we call the random_range method on the random number generator. This method is defined by 
    // // the Rng trait that we brought into scope with the use rand::rng
    let secret_number = rand::rng().random_range(1..=100);

    println!("The secret number is: {secret_number}");

    loop{
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
            // // `read_line` is a method on the standard input handle to get input from the user
            // // We're also passing &mut guess as the argument to read_line to tell it what string to stroe the user input in
            // // the `&` indicates that this argument is a reference to the variable guess
            .read_line(&mut guess)

            // // Will crash and give you a indicator why
            .expect("Failed to read line");
            
        
        // // `trim()` removes any leading and trailing whitespace
        // // Since we assigned the result to `let guess: u32`, Rust infers that `parse()` should convert the string to a u32
        // // `.expect()` will terminate the program if the converstion failes and display the text message inside
        let guess: u32 = guess.trim().parse().expect("Please type a number!");

        println!("You guessed: {}", guess);

        // // The `cmp` method compares two values and can be called on anything that can be compared. Here it 
        // // compares `guess` to `secret_number`. Then ir treutns a variant of the Ordering enum we brought into scope
        // //
        // // We use a `match` expression to decide what to do next based on whcih variant of Ordering was returned. This
        // // approach makes it more robust. 
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => println!("You win!"),
        }
    }
}

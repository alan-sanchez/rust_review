fn main() {
    println!("Hello, world!");

    another_function();

    print_labeled_measurement(15, 'f');

    macro_expression();

    let x = five();
    println!("The value of x is: {x}");

    let y = plus_one(x);
    println!("The value of x plus one is: {y}")


}

fn another_function(){
    println!("Another function.")
}

// // In the function signatures, you MUST decluare the type of each parameter
fn print_labeled_measurement(value:u32, unit_label:char){
    println!("The measurement value is: {value}{unit_label}")
}


fn macro_expression(){
    let y = {
        let x = 3;
        x + 1 
    };

    println!("The value of y is: {y}");
}

// // Functions with return values. We don't name return values, but we must delcare their type 
// // after an arrow (->). Most functions return the last expression implicity. 
fn five() -> u32 {
    5 // no semicolon because it's an expression whose value we want to return
}


fn plus_one(x:u32) -> u32 {
    x + 1 // If we were to add a `;` then the expression becomes a statement, raising an error
}
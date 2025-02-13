fn main() {
    // // Conditional statement
    let number = 7;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }


    // // Another example of a series of conditional statments
    let number = 6;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // // Using if in a let Statement
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");

    // // // Running a loop
    // loop {
    //     println!("again!");
    // }

    // // Returning values from loops
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2; // this returns to the counter multiplied by 2. Notice that is needs a semi-colon
        }
    };
    println!("The result is {result}");


    // // Loop lables to help distinguis mulitple loops
    let mut count = 0;
    // // Define a lableled outer loop called 'counting_up
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;
        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");


    // // Conditional Loops w/ While
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("the value is: {}", a[index]);
        index += 1;
    }

    // // Looping through each element 
    let a = ['a', 'c', 'b', 'f', 'a'];
    for element in a {
        println!("the value is: {element}");
    }


    // Loop through a range:
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}

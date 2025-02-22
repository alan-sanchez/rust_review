fn main() {
    // Create a new String variable `s1` and initialize it with "hello".
    let s1 = String::from("hello");

    // Pass a reference to `s1` to the `calculate_length` function.
    // This allows `calculate_length` to borrow `s1` without taking ownership.
    let len = calculate_length(&s1);

    // Print the original string and its calculated length.
    println!("The length of '{s1}' is {len}.");
}

// Function that takes a reference to a String (`s: &String`) and returns its length.
fn calculate_length(s: &String) -> usize {
    s.len()  // `.len()` method returns the number of bytes in the String.
}

fn main() {
    // // Create a new String variable `s1` and initialize it with "hello".
    let s1 = String::from("hello");

    // // Pass a reference to `s1` to the `calculate_length` function. the `&s1` syntax lets us create a reference
    // // that refers to the value of s1 but does not own it. 
    let len = calculate_length(&s1);

    // // Print the original string and its calculated length.
    println!("The length of '{s1}' is {len}.");


    // // Mutable Reference // // 
    let mut s = String::from("Hola");

    change(&mut s);
    println!("{s}");

}

// // Function that takes a reference to a String (`s: &String`) and returns its length.
fn calculate_length(s: &String) -> usize {
    s.len()  // `.len()` method returns the number of bytes in the String.
} // // Here, s goes out of scope. But because s does not have ownership of what
  // // it refers to, the value is not dropped.


  // // 
  fn change(some_string: &mut String) {
    some_string.push_str(", mundo");
  }
fn main() {

    // // We create an instance of the struct by specifying concrete values of eahc of the fields. (key:value pairs)
    // // 
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    // // To get a specific value from a struct, we use the dot notation, `user1.email`. Only when the instance is mutable
    // // can we change the value
    user1.email = String::from("anotheremail@example.com");
}


// // To define a strut, we ter the keyword `struct` and name the entire struct. Inside the curly brackets,
// // we define the names and types of the pieces of data, which we call fields
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
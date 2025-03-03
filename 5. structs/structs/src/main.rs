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

    // // Creating instance from other instance with Struct Update Syntax
    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };

    // // Creating another instance with less code
    let user3 = User{
        active: user2.active,
        username: user2.username,
        ..user2
    };

    // // Tuple structs
    let black = Color(0,0,0);
    let origin = Point(0,0,0);

    // // 
    let subject = AlwaysEqual;
}


// // To define a strut, we ter the keyword `struct` and name the entire struct. Inside the curly brackets,
// // we define the names and types of the pieces of data, which we call fields
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// // using the Field Init Shorthand syntax to rereithe build_user so it behaves exatly the same but doesn't 
// // have the repeititions of username and email
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

// // Using Tuple structs without hamed fields to create different types
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// // Unit-Like Structs without any fields
struct AlwaysEqual;
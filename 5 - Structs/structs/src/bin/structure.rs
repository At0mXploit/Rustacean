// Defining Struct

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count = u64,
}

// Tuple Struct
struct Color (i32, i32, i32);
struct Point(i32, i32, i32);

// Unit Struct - Struct that dont have any fields 
struct AlwaysEqual;

fn main() {
    let mut user1 = User {
        email: String::from("someone@gmail.com");
        username: String::from("someoneusername");
        active: true 
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@gmail.com");

    let user2 = User {
    // Setting new value for only email other all from User1
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };

    // Above effect can also be achieved using syntax ..
    let user3 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    // Using Tuple Structs 
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let subject = AlwaysEqual;
}

// Function that returns a User instance with given email and username 
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }

// Using init 
fn build_user_using_init(email: String, username: String) -> User {
    User {
        active: true,
        username, // Because parameter name and field name hash same name we can just write email
                  // instead of email:email
        email,
        sign_in_count: 1,
    }
}

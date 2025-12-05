// Lifetimes ensure that the data referenced by a struct is valid for as 
// long as the struct is. Let’s say you try to store a reference 
// in a struct without specifying lifetimes, like the following; this won’t work:

struct User {
    active: bool,
    username: &str,
    email: &str,
    sign_in_count: u64,
}

fn main() {
    let user1 = User {
        active: true,
        username: "someusername123",
        email: "someone@example.com",
        sign_in_count: 1,
    };
}

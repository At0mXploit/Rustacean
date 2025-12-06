fn main() {
    // Option<T> has two possibilities
    // 1. Some(value) - Has a value
    // 2. None - Has no value
    
    // Creating Some (has value)
    let has_value = Some(42); // Option<i32> with value 42
    let has_name = Some("Alice")  // Option<&str> with value "Alice"
    
    // Creating None (no value)
    let no_value: Option<i32> = None; // Option<i32> with no value
    let no_name = Option<String> = None; // Option<String> with no value
    
    println!("has_value", has_value); // Prints Some(42)
    println!("no_value", no_value); // Prints None
    
    // PROBLEM in other languages (Java/Python/C++):
    // String name = null;
    // print(name.length()); // CRASH! NullPointerException
}

// Rust Solution
fn get_user_name(user_id: i32) -> Option<String> {
    if user_id == 1{
        Some(String::from('Alice')) // User exists
    } else {
        None // User doesn't exists
    }
}

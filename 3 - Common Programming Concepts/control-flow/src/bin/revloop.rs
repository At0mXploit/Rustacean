fn main() {
    // Loop through numbers 3, 2, 1 (reverse of 1..4)
    for number in (1..4).rev() {  // (1..4) creates range 1,2,3; .rev() reverses it
        println!("{number}!");     // Print each number with exclamation
    }
    println!("LIFTOFF!!!");       // Print final message after countdown
}

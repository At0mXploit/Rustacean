// Enum variants can hold data directly
enum IpAddr {
    V4(String),
    V6(String),
}

fn main() {
    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));
    
    // Note: Each variant becomes a constructor function
    // IpAddr::V4() is a function that returns IpAddr
    
    println!("Created IP addresses using enum constructors");
}

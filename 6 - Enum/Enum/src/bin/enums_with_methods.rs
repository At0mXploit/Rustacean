enum Message {
    Write(String),
}

impl Message {
    // Method on the enum
    fn call(&self) {
        // We'll learn pattern matching to access data inside
        println!("Message called!");
    }
}

fn main() {
    let m = Message::Write(String::from("hello"));
    m.call();  // Calling method on enum instance
}

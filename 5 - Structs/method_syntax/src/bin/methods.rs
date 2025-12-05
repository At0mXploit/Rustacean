// Methods are functions defined within a struct (or enum/trait object) context that operate on instances of that type, with self representing the instance being operated upon
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    }

    println!("The area is {}", rect1.area());
}

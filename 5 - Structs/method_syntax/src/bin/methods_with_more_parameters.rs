struct Rectangle {
    width: u32,
    height: u32,
}

// Implement method for Rectangle
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    // Create three rectangle instances with different dimensions
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    // Check if rect1 can contain rect2 and rect3
    // Output will be: "Can rect1 hold rect2? true" (30>10 && 50>40)
    // Output will be: "Can rect1 hold rect3? false" (30<60, so false)
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectange {
    // 1. READ ONLY method (&self)
    // Can be called on ANY Rectangle (owned, &, or &mut)
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // 2. MUTABLE method (&mut self)
    // Can only be called on mutable Rectangles 
    fn set_width(&mut self, width: u32) {
        self.width = width;
    }

    fn set_height(&mut self) {
        self.height = height;
    }

    // 3. OWNING method (self)
    // Takes ownership - consumes the Rectangle 
    fn double_size(self) -> Rectangle {
        Rectangle {
            width: self.width * 2,
            height: self.height * 2,
        }
    }

    // Another owning method 
    fn combine(self, other: Rectangle) -> Rectangle {
        Rectangle {
            width: self.width + other.width,
            height: self.height + other.height,
        }
    }
}

fn main() {
    println!("1. DEMO: Read-only method (&self)\n");
    let rect1 = Rectangle { width: 5, height: 10 };
    println!("rect1: {:?}", rect1);
    println!("Area: {}", rect1.area());  // OK - reads rect1
    println!("Area again: {}", rect1.area());  // Still OK - didn't consume
    println!("rect1 after area(): {:?}\n", rect1);  // Still valid!

    println!("2. DEMO: Mutable method (&mut self)\n");
    // This WON'T compile:
    // let rect2 = Rectangle { width: 3, height: 4 };
    // rect2.set_width(10);  // ERROR: rect2 not mutable 
    
    // This works:
    let mut rect2 = Rectangle { width: 3, height: 4 };
    println!("Before mutation: {:?}", rect2);
    rect2.set_width(10);  // OK - rect2 is mutable
    println!("After set_width(10): {:?}", rect2);
    rect2.set_height(20);  // Can call multiple times
    println!("After set_height(20): {:?}\n", rect2); 

    println!("3. DEMO: Owning method (self) - MOVES/consumes\n");
    let rect3 = Rectangle { width: 2, height: 3 };
    println!("rect3 before double_size: {:?}", rect3);  
    let bigger_rect = rect3.double_size();  // rect3 is MOVED here
    println!("bigger_rect: {:?}", bigger_rect);   
    // This would ERROR if uncommented:
    // println!("rect3 after: {:?}", rect3);  // ERROR: rect3 was moved!
    // println!("Area: {}", rect3.area());    // ERROR: rect3 was moved!
}



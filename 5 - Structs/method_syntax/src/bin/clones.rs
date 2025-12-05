// Use Copy for Numnbers. Boolean, Characters, Simple structs 
// Use Clone for Strings Lists Vec<T> 


#[derive(Copy, Clone)] // 1. COPY TYPES (automatic copy)

struct Point {
    x: i32,
    y: i32
}

fn main() {
    println!("=== SIMPLE COPY EXAMPLE ===");
    let p1 = Point { x: 5, y: 10 };
    let p2 = p1;  // Automatically COPIES (not moves!)

    // 2. CLONE TYPES (must explicitly clone)
    #[derive(Clone)]  // Can clone, but not copy
    struct Person {
        name: String,  // String is NOT Copy!
        age: u8,
    }

    let person1 = Person {
        name: String::from("Alice"),
        age: 30,
    };
    
    // Must explicitly clone
    let person2 = person1.clone();  // Explicit clone

    // 3. NO COPY OR CLONE
    struct Car {
        brand: String,
    }
    
    let car1 = Car {
        brand: String::from("Toyota"),
    };
    
    let car2 = car1;  // This MOVES (not copies!)
    
    // println!("car1: {}", car1.brand);  // ERROR! car1 was moved
    println!("car2: {}", car2.brand);  //  Only car2 works
}



// Define a struct named 'Point' to represent a 2D coordinate
// Contains two integer fields: x and y
struct Point {
    x: i32,  // x-coordinate (32-bit signed integer)
    y: i32,  // y-coordinate (32-bit signed integer)
}

fn main() {
    // Create a mutable instance of Point called 'p'
    // 'mut' allows us to modify p's fields later
    // Initialize it at coordinates (0, 0)
    let mut p = Point { x: 0, y: 0 };

    // Create a mutable reference specifically to the 'x' field of point 'p'
    // 'x' now borrows p.x mutably - we can change p.x through this reference
    let x = &mut p.x;

    // Dereference 'x' using * to access and modify the actual value
    // Equivalent to: p.x = p.x + 1;
    *x += 1;

    // Print both coordinates of point 'p'
    // Note: The mutable borrow of p.x (through 'x') ended at line 19,
    // so we can now read both p.x and p.y
    println!("{}, {}", p.x, p.y);
    // Expected output: "1, 0" (x increased by 1, y remains 0)
}

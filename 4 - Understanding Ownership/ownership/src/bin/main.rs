fn main() {
    println!("=== RUST OWNERSHIP CONCEPTS ===\n");
    
    // ========================================================================
    // 1. WHAT IS OWNERSHIP?
    // ========================================================================
    println!("1. WHAT IS OWNERSHIP?");
    println!("   - Rust's memory management system (no GC, no manual malloc/free)");
    println!("   - Set of rules enforced at compile time");
    println!("   - Prevents memory errors: double frees, use-after-free, etc.\n");
    
    // ========================================================================
    // 2. THE STACK AND THE HEAP
    // ========================================================================
    println!("2. STACK vs HEAP:");
    
    // Stack example - fixed size, fast
    let x = 5;  // i32, known size, goes on stack
    let y = 3.14;  // f64, known size, goes on stack
    println!("   Stack values: x = {}, y = {}", x, y);
    
    // Heap example - dynamic size, slower
    let s = String::from("hello");  // String, dynamic size, data on heap
    println!("   Heap value: s = '{}' (pointer on stack, data on heap)", s);
    
    println!("   Key differences:");
    println!("   - Stack: LIFO, fixed size, fast, automatic allocation");
    println!("   - Heap: dynamic size, slower, manual allocation/release\n");
    
    // ========================================================================
    // 3. OWNERSHIP RULES
    // ========================================================================
    println!("3. THREE OWNERSHIP RULES:");
    println!("   1. Each value has an owner");
    println!("   2. Only one owner at a time");
    println!("   3. When owner goes out of scope, value is dropped\n");
    
    // ========================================================================
    // 4. VARIABLE SCOPE
    // ========================================================================
    println!("4. VARIABLE SCOPE:");
    {
        let inner_var = "I exist only in this block";
        println!("   Inside block: {}", inner_var);
        // inner_var will be dropped here
    }
    // println!("Outside: {}", inner_var); // ERROR! inner_var doesn't exist here
    
    // ========================================================================
    // 5. STRING TYPE - HEAP ALLOCATED
    // ========================================================================
    println!("\n5. STRING TYPE (Heap allocated):");
    
    // String literal (immutable, in binary)
    let literal = "hello";  // &str type, immutable
    println!("   String literal: '{}' (immutable, in binary)", literal);
    
    // String type (mutable, heap allocated)
    let mut string = String::from("hello");
    string.push_str(", world!");
    println!("   String type: '{}' (mutable, heap allocated)", string);
    
    // ========================================================================
    // 6. MOVE SEMANTICS
    // ========================================================================
    println!("\n6. MOVE SEMANTICS (Key Concept!):");
    
    let s1 = String::from("hello");
    println!("   Created s1 = '{}'", s1);
    
    // This MOVES ownership from s1 to s2
    let s2 = s1;  // s1 is INVALIDATED after this line
    
    println!("   After 'let s2 = s1':");
    println!("   - s2 = '{}' (valid)", s2);
    // println!("   - s1 = '{}' (ERROR: borrow of moved value)", s1);
    println!("   - s1 is INVALID (ownership moved to s2)");
    
    // Why? To prevent double-free!
    // Both s1 and s2 would try to free the same memory when going out of scope
    
    // ========================================================================
    // 7. CLONING (Deep Copy)
    // ========================================================================
    println!("\n7. CLONING (Explicit deep copy):");
    
    let s3 = String::from("hello");
    let s4 = s3.clone();  // Explicit deep copy - both heap data AND metadata
    
    println!("   After cloning:");
    println!("   - s3 = '{}' (still valid)", s3);
    println!("   - s4 = '{}' (copy of s3)", s4);
    println!("   - Both are valid independent strings");
    println!("   - Expensive operation - copies heap data\n");
    
    // ========================================================================
    // 8. COPY TRAIT (Stack-only types)
    // ========================================================================
    println!("8. COPY TRAIT (Stack-only types):");
    
    let x1 = 5;
    let x2 = x1;  // This COPIES (not moves) because i32 implements Copy
    
    println!("   Integer example:");
    println!("   - x1 = {} (still valid)", x1);
    println!("   - x2 = {} (copy of x1)", x2);
    
    let c1 = 'A';
    let c2 = c1;  // char implements Copy
    println!("   Char example:");
    println!("   - c1 = '{}' (still valid)", c1);
    println!("   - c2 = '{}' (copy of c1)", c2);
    
    let t1 = (1, 2);  // Tuple with Copy types
    let t2 = t1;      // Copies
    println!("   Tuple example (i32, i32):");
    println!("   - t1 = {:?} (still valid)", t1);
    println!("   - t2 = {:?} (copy of t1)", t2);
    
    // Types that DO NOT implement Copy:
    // - String
    // - Vec<T>
    // - Box<T>
    // - Any type that needs heap allocation
    
    // ========================================================================
    // 9. OWNERSHIP AND FUNCTIONS
    // ========================================================================
    println!("\n9. OWNERSHIP AND FUNCTIONS:");
    
    let s_func = String::from("hello");
    println!("   Before function call: s_func = '{}'", s_func);
    
    takes_ownership(s_func);  // s_func MOVED into function
    // println!("After: {}", s_func); // ERROR! s_func is invalid here
    
    let x_func = 5;
    makes_copy(x_func);  // x_func COPIED into function
    println!("   After function (i32): x_func = {} (still valid - Copy trait)", x_func);
    
    // ========================================================================
    // 10. RETURN VALUES AND SCOPE
    // ========================================================================
    println!("\n10. RETURN VALUES AND SCOPE:");
    
    let s_returned = gives_ownership();  // Takes ownership of return value
    println!("   Received from function: '{}'", s_returned);
    
    let s_to_pass = String::from("hello");
    let s_back = takes_and_gives_back(s_to_pass);  // Pass and get back ownership
    println!("   Got back: '{}'", s_back);
    
    // ========================================================================
    // 11. TUPLES FOR MULTIPLE RETURNS
    // ========================================================================
    println!("\n11. TUPLES FOR MULTIPLE RETURN VALUES:");
    
    let s_tuple = String::from("hello");
    let (s_result, len) = calculate_length(s_tuple);
    println!("   String: '{}', Length: {}", s_result, len);
    
    // ========================================================================
    // 12. REASSIGNMENT AND DROP
    // ========================================================================
    println!("\n12. REASSIGNMENT AND DROP:");
    
    let mut s_reassign = String::from("hello");
    println!("   Created: '{}'", s_reassign);
    
    s_reassign = String::from("ahoy");  // Original "hello" is DROPPED here!
    println!("   Reassigned: '{}'", s_reassign);
    println!("   Original 'hello' was automatically freed (drop called)");
    
    println!("\n=== SUMMARY ===");
    println!("1. Ownership prevents memory errors at compile time");
    println!("2. Move semantics prevent double-free errors");
    println!("3. Clone for explicit deep copies");
    println!("4. Copy trait for cheap stack copies");
    println!("5. Functions take ownership (unless using references)");
    println!("6. Drop is called automatically when values go out of scope");
}

// ============================================================================
// Helper functions demonstrating ownership transfer
// ============================================================================

fn takes_ownership(some_string: String) {
    println!("   Inside function: received '{}'", some_string);
    // some_string will be dropped here when function ends
}

fn makes_copy(some_integer: i32) {
    println!("   Inside function: received {}", some_integer);
    // some_integer is a copy, original still exists
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string  // Ownership is transferred to caller
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string  // Ownership is transferred back to caller
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)  // Return both the String and its length
}

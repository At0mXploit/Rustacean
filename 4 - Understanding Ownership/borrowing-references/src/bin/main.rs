fn main() {
    println!("=== RUST REFERENCES AND BORROWING ===\n");
    
    // ========================================================================
    // 1. INTRODUCTION TO REFERENCES
    // ========================================================================
    println!("1. INTRODUCTION TO REFERENCES:");
    println!("   - References allow you to refer to a value without taking ownership");
    println!("   - Created using the & operator");
    println!("   - Guaranteed to point to valid data (no null pointers)\n");
    
    // ========================================================================
    // 2. IMMUTABLE REFERENCES (BORROWING)
    // ========================================================================
    println!("2. IMMUTABLE REFERENCES (Borrowing):");
    
    let s1 = String::from("hello");
    println!("   Created s1 = '{}'", s1);
    
    // Pass a reference to s1 (borrow it)
    let len = calculate_length(&s1);
    
    println!("   Length of '{}' is {}", s1, len);
    println!("   s1 is STILL valid - we only borrowed it!");
    
    // Multiple immutable references allowed
    let r1 = &s1;
    let r2 = &s1;
    let r3 = &s1;
    println!("   Multiple immutable references allowed:");
    println!("   r1 = '{}', r2 = '{}', r3 = '{}'", r1, r2, r3);
    
    // ========================================================================
    // 3. MUTABLE REFERENCES
    // ========================================================================
    println!("\n3. MUTABLE REFERENCES:");
    
    let mut s2 = String::from("hello");
    println!("   Created mutable s2 = '{}'", s2);
    
    // Create a mutable reference
    change(&mut s2);
    println!("   After mutation: s2 = '{}'", s2);
    
    // ========================================================================
    // 4. RESTRICTIONS ON MUTABLE REFERENCES
    // ========================================================================
    println!("\n4. RESTRICTIONS ON MUTABLE REFERENCES:");
    
    let mut s3 = String::from("hello");
    
    // First mutable borrow - OK
    let r1 = &mut s3;
    println!("   Created first mutable reference r1");
    r1.push_str(", world");
    println!("   Modified through r1: '{}'", r1);
    
    // r1 goes out of scope here (last use), so we can create another
    let r2 = &mut s3;
    r2.push_str(" again!");
    println!("   Created second mutable reference r2 after r1 dropped");
    println!("   Modified through r2: '{}'", r2);
    
    // ========================================================================
    // 5. DATA RACES PREVENTION
    // ========================================================================
    println!("\n5. PREVENTING DATA RACES:");
    
    let mut s4 = String::from("hello");
    
    // This would cause a compile error (commented out):
    /*
    let r1 = &mut s4;
    let r2 = &mut s4;  // ERROR: cannot borrow as mutable more than once
    println!("{}, {}", r1, r2);
    */
    println!("   Cannot have two mutable references at the same time");
    println!("   This prevents data races at compile time!");
    
    // ========================================================================
    // 6. MUTABLE AND IMMUTABLE REFERENCES CAN'T COEXIST
    // ========================================================================
    println!("\n6. MUTABLE AND IMMUTABLE REFERENCES:");
    
    let mut s5 = String::from("hello");
    
    let immut1 = &s5;  // Immutable borrow
    let immut2 = &s5;  // Another immutable borrow - OK
    
    println!("   Immutable references: '{}', '{}'", immut1, immut2);
    
    // After last use of immutable references, we can create mutable one
    let mut_ref = &mut s5;
    mut_ref.push_str(" world");
    println!("   After immutable refs done: mutable ref = '{}'", mut_ref);
    
    // ========================================================================
    // 7. REFERENCE SCOPES
    // ========================================================================
    println!("\n7. REFERENCE SCOPES (NLL - Non-Lexical Lifetimes):");
    
    let mut s6 = String::from("hello");
    
    let ref1 = &s6;  // Immutable borrow
    let ref2 = &s6;  // Another immutable borrow
    
    println!("   Using immutable refs: {}, {}", ref1, ref2);
    // ref1 and ref2 are no longer used after this point
    
    let ref3 = &mut s6;  // This is OK! ref1 and ref2 are "dead"
    ref3.push_str(" mutable");
    println!("   Mutable reference after immutable ones done: '{}'", ref3);
    
    // ========================================================================
    // 8. DANGLING REFERENCES PREVENTION
    // ========================================================================
    println!("\n8. PREVENTING DANGLING REFERENCES:");
    
    // This would create a dangling reference (commented out):
    /*
    let dangling_ref = dangle();  // ERROR: returns reference to local variable
    */
    
    let valid_string = no_dangle();  // Returns owned String
    println!("   Safe function returns owned value: '{}'", valid_string);
    println!("   Rust prevents returning references to local variables");
    
    // ========================================================================
    // 9. BORROWING WITH FUNCTIONS
    // ========================================================================
    println!("\n9. BORROWING WITH FUNCTIONS:");
    
    let s7 = String::from("I'm borrowed");
    borrow_string(&s7);
    println!("   After borrowing in function: s7 = '{}'", s7);
    
    let mut s8 = String::from("I can be modified");
    borrow_mut_string(&mut s8);
    println!("   After mutable borrowing: s8 = '{}'", s8);
    
    // ========================================================================
    // 10. BORROWING RULES SUMMARY
    // ========================================================================
    println!("\n=== BORROWING RULES SUMMARY ===");
    println!("1. At any given time, you can have EITHER:");
    println!("   - One mutable reference");
    println!("   - OR any number of immutable references");
    println!("2. References must always point to valid data");
    println!("3. The compiler tracks reference lifetimes");
    println!("4. Prevents: data races, use-after-free, dangling pointers");
    
    // ========================================================================
    // 11. PRACTICAL EXAMPLES
    // ========================================================================
    println!("\n=== PRACTICAL EXAMPLES ===");
    
    // Example 1: Reading without ownership
    let text = String::from("Hello Rustaceans!");
    print_first_word(&text);
    print_length(&text);
    println!("   Original still available: '{}'", text);
    
    // Example 2: Modifying through reference
    let mut scores = vec![85, 92, 78];
    println!("   Original scores: {:?}", scores);
    add_bonus(&mut scores, 5);
    println!("   After bonus: {:?}", scores);
    
    // Example 3: Multiple reads
    let data = vec![1, 2, 3, 4, 5];
    let sum = calculate_sum(&data);
    let avg = calculate_average(&data);
    println!("   Data: {:?}, Sum: {}, Average: {}", data, sum, avg);
}

// ============================================================================
// Helper Functions
// ============================================================================

// Immutable reference parameter
fn calculate_length(s: &String) -> usize {
    // s is a reference, we don't own it
    s.len()
    // s is not dropped here because we don't own it
}

// Mutable reference parameter
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// Function that would create dangling reference (WON'T COMPILE)
// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s  // ERROR: returns reference to local variable
// }

// Safe alternative - return owned value
fn no_dangle() -> String {
    let s = String::from("hello");
    s  // Ownership transferred to caller
}

// Borrowing for reading
fn borrow_string(s: &String) {
    println!("   Inside borrow_string: '{}' (length: {})", s, s.len());
}

// Borrowing for mutation
fn borrow_mut_string(s: &mut String) {
    s.push_str(" through function");
}

// Multiple functions can borrow immutably
fn print_first_word(s: &str) {
    if let Some(first_word) = s.split_whitespace().next() {
        println!("   First word: '{}'", first_word);
    }
}

fn print_length(s: &str) {
    println!("   Length: {}", s.len());
}

// Mutable borrowing for modification
fn add_bonus(scores: &mut Vec<i32>, bonus: i32) {
    for score in scores.iter_mut() {
        *score += bonus;
    }
}

// Immutable borrowing for calculations
fn calculate_sum(numbers: &Vec<i32>) -> i32 {
    numbers.iter().sum()
}

fn calculate_average(numbers: &Vec<i32>) -> f64 {
    let sum: i32 = numbers.iter().sum();
    sum as f64 / numbers.len() as f64
}


fn main() {
    println!("=== RUST SLICES ===\n");
    
    // ========================================================================
    // 1. INTRODUCTION TO SLICES
    // ========================================================================
    println!("1. WHAT ARE SLICES?");
    println!("   - References to contiguous sequence of elements");
    println!("   - Do not have ownership (like references)");
    println!("   - Provide safe view into collections\n");
    
    // ========================================================================
    // 2. THE PROBLEM WITH INDEX-BASED APPROACH
    // ========================================================================
    println!("2. THE PROBLEM: INDEX-BASED APPROACH");
    
    let mut s1 = String::from("hello world");
    println!("   Created s1 = '{}'", s1);
    
    // Problematic approach: returning just an index
    let word_index = first_word_index(&s1);  // Returns 5
    println!("   First word ends at index: {}", word_index);
    
    // Now if we modify the string...
    s1.clear();  // String becomes empty
    
    println!("   After s1.clear(): s1 = '{}'", s1);
    println!("   Problem: word_index = {} is now INVALID!", word_index);
    println!("   Index is disconnected from actual data");
    
    // ========================================================================
    // 3. STRING SLICES TO THE RESCUE
    // ========================================================================
    println!("\n3. STRING SLICES SOLUTION:");
    
    let s2 = String::from("hello world");
    println!("   Created s2 = '{}'", s2);
    
    // Create string slices using range syntax
    let hello = &s2[0..5];    // Explicit range
    let world = &s2[6..11];   // From index 6 to 10
    println!("   Slice 1 (hello): '{}'", hello);
    println!("   Slice 2 (world): '{}'", world);
    
    // How slices work internally:
    println!("   Slice 'hello' contains:");
    println!("   - Pointer to byte at index 0");
    println!("   - Length: 5 bytes");
    
    // ========================================================================
    // 4. RANGE SYNTAX SHORTHANDS
    // ========================================================================
    println!("\n4. RANGE SYNTAX SHORTHANDS:");
    
    let s3 = String::from("rust");
    
    // All these are equivalent:
    let slice1 = &s3[0..4];
    let slice2 = &s3[..4];     // Start at 0 implied
    let slice3 = &s3[0..];     // Go to end implied
    let slice4 = &s3[..];      // Entire string
    
    println!("   String: '{}'", s3);
    println!("   &s3[0..4]  = '{}'", slice1);
    println!("   &s3[..4]   = '{}'", slice2);
    println!("   &s3[0..]   = '{}'", slice3);
    println!("   &s3[..]    = '{}'", slice4);
    
    // ========================================================================
    // 5. FIRST_WORD FUNCTION WITH SLICES
    // ========================================================================
    println!("\n5. FIRST_WORD FUNCTION (Slice Version):");
    
    let s4 = String::from("hello beautiful world");
    let first = first_word(&s4);
    println!("   String: '{}'", s4);
    println!("   First word: '{}'", first);
    
    // The slice is connected to the original data
    println!("   Slice is a REFERENCE to part of s4");
    println!("   Compiler ensures slice stays valid");
    
    // ========================================================================
    // 6. COMPILER PREVENTS ERRORS
    // ========================================================================
    println!("\n6. COMPILER-PREVENTED ERROR EXAMPLE:");
    
    let mut s5 = String::from("hello world");
    println!("   Created mutable s5 = '{}'", s5);
    
    let word = first_word(&s5);  // Immutable borrow occurs
    
    println!("   First word: '{}'", word);
    
    // This would cause compile error (commented out):
    /*
    s5.clear();  // ERROR: cannot borrow as mutable
    println!("After clear: {}", word);  // word would be invalid!
    */
    
    println!("   Cannot call s5.clear() while word exists");
    println!("   Compiler prevents use-after-invalidation!");
    
    // After word is no longer used, we can modify
    println!("   (Dropping word reference...)");
    let new_word = "test";  // word is no longer used after this
    s5.clear();  // Now allowed!
    s5.push_str("new content");
    println!("   After dropping old reference: s5 = '{}'", s5);
    
    // ========================================================================
    // 7. STRING LITERALS AS SLICES
    // ========================================================================
    println!("\n7. STRING LITERALS ARE SLICES:");
    
    let literal = "Hello, world!";  // Type: &str
    println!("   String literal: '{}'", literal);
    println!("   Type: &str (immutable string slice)");
    println!("   Stored in binary, points to specific memory location");
    
    // Can take slices of string literals too
    let hello_lit = &literal[0..5];
    let world_lit = &literal[7..12];
    println!("   Slice of literal (hello): '{}'", hello_lit);
    println!("   Slice of literal (world): '{}'", world_lit);
    
    // ========================================================================
    // 8. FLEXIBLE FUNCTION SIGNATURES
    // ========================================================================
    println!("\n8. FLEXIBLE FUNCTION SIGNATURES:");
    
    let my_string = String::from("hello world");
    let my_literal = "hello world";
    
    // All these work with the same function!
    println!("   Testing flexible_first_word:");
    println!("   With String reference: '{}'", flexible_first_word(&my_string));
    println!("   With String slice: '{}'", flexible_first_word(&my_string[..]));
    println!("   With partial slice: '{}'", flexible_first_word(&my_string[0..5]));
    println!("   With literal: '{}'", flexible_first_word(my_literal));
    println!("   With literal slice: '{}'", flexible_first_word(&my_literal[..6]));
    
    // ========================================================================
    // 9. ARRAY SLICES
    // ========================================================================
    println!("\n9. ARRAY SLICES:");
    
    let arr = [1, 2, 3, 4, 5];
    println!("   Array: {:?}", arr);
    
    let arr_slice = &arr[1..4];  // Type: &[i32]
    println!("   Slice [1..4]: {:?}", arr_slice);
    println!("   Slice type: &[i32]");
    
    // Array slices work the same way
    println!("   Contains reference to first element and length");
    
    // ========================================================================
    // 10. PRACTICAL EXAMPLES
    // ========================================================================
    println!("\n10. PRACTICAL EXAMPLES:");
    
    // Example 1: Extracting parts of data
    let data = "2023-12-04T15:30:00";
    let date = &data[0..10];
    let time = &data[11..19];
    println!("   Timestamp: {}", data);
    println!("   Date: {}, Time: {}", date, time);
    
    // Example 2: Multiple word extraction
    let sentence = "The quick brown fox jumps";
    let words = extract_words(sentence);
    println!("   Sentence: '{}'", sentence);
    println!("   Word 1: '{}', Word 2: '{}', Word 3: '{}'", 
             words.0, words.1, words.2);
    
    // Example 3: Array manipulation with slices
    let numbers = [10, 20, 30, 40, 50, 60, 70, 80, 90];
    let middle = &numbers[3..7];
    println!("   Numbers: {:?}", numbers);
    println!("   Middle slice [3..7]: {:?}", middle);
    
    // ========================================================================
    // 11. SLICE SAFETY
    // ========================================================================
    println!("\n11. SLICE SAFETY FEATURES:");
    
    let s6 = String::from("hello");
    
    // These would panic at runtime (commented out):
    /*
    let bad1 = &s6[0..10];  // Index out of bounds
    let bad2 = &s6[0..1];   // Valid but...
    let bad3 = &s6[1..2];   // Still valid bytes
    */
    
    // UTF-8 boundary safety
    let unicode = "नमस्ते";  // Hindi word
    println!("   Unicode string: {}", unicode);
    println!("   Can't slice in middle of multibyte character");
    println!("   &unicode[0..3] would panic (invalid UTF-8 boundary)");
    
    // Safe slicing
    if let Some(slice) = safe_slice(unicode, 0, 3) {
        println!("   Safe slice [0..3]: {}", slice);
    }
    
    println!("\n=== SLICE SUMMARY ===");
    println!("1. Slices are references to contiguous data");
    println!("2. String slices: &str");
    println!("3. Array slices: &[T]");
    println!("4. Prevent index/data synchronization bugs");
    println!("5. Compiler ensures validity through lifetimes");
    println!("6. Enable flexible, efficient APIs");
}

// ============================================================================
// Problematic approach: Returns index only
// ============================================================================

/// Returns the index of the end of the first word
/// PROBLEM: Index can become invalid if string changes!
fn first_word_index(s: &String) -> usize {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;  // Return index where space found
        }
    }
    
    s.len()  // No space found, entire string is one word
}

// ============================================================================
// Better approach: Returns string slice
// ============================================================================

/// Returns a slice of the first word in a string
/// BETTER: Returns reference that's tied to original data
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            // Return slice from start to space
            return &s[0..i];
        }
    }
    
    // No space found, return entire string slice
    &s[..]
}

// ============================================================================
// Best approach: Accepts &str (works with both String and &str)
// ============================================================================

/// Most flexible version: accepts any string slice
/// Can be called with String, &str, or string slices
fn flexible_first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    
    &s[..]
}

// ============================================================================
// Helper Functions
// ============================================================================

/// Extracts first three words from a string
/// Demonstrates multiple slice returns
fn extract_words(s: &str) -> (&str, &str, &str) {
    let bytes = s.as_bytes();
    let mut word_starts = Vec::new();
    let mut word_ends = Vec::new();
    
    // Find word boundaries
    let mut in_word = false;
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            if in_word {
                word_ends.push(i);
                in_word = false;
            }
        } else {
            if !in_word {
                word_starts.push(i);
                in_word = true;
            }
        }
    }
    
    if in_word {
        word_ends.push(s.len());
    }
    
    // Extract up to 3 words
    let word1 = if word_starts.len() > 0 {
        &s[word_starts[0]..word_ends[0]]
    } else {
        ""
    };
    
    let word2 = if word_starts.len() > 1 {
        &s[word_starts[1]..word_ends[1]]
    } else {
        ""
    };
    
    let word3 = if word_starts.len() > 2 {
        &s[word_starts[2]..word_ends[2]]
    } else {
        ""
    };
    
    (word1, word2, word3)
}

/// Safely slices a string with UTF-8 boundary checking
/// Returns None if slice would break UTF-8 characters
fn safe_slice(s: &str, start: usize, end: usize) -> Option<&str> {
    // Check bounds
    if end > s.len() || start > end {
        return None;
    }
    
    // Check UTF-8 character boundaries
    if !s.is_char_boundary(start) || !s.is_char_boundary(end) {
        return None;
    }
    
    Some(&s[start..end])
}

// ============================================================================
// Advanced Examples
// ============================================================================

fn demonstrate_second_word() {
    println!("\n=== ADVANCED: SECOND WORD FUNCTION ===");
    
    let s = String::from("hello world again");
    
    // Second word function using slices
    if let Some(second) = second_word(&s) {
        println!("   String: '{}'", s);
        println!("   Second word: '{}'", second);
    }
    
    // What happens with short strings?
    let short = "hello";
    if let Some(second) = second_word(short) {
        println!("   Short string second word: '{}'", second);
    } else {
        println!("   Short string has no second word");
    }
}

/// Returns the second word in a string, if it exists
fn second_word(s: &str) -> Option<&str> {
    let bytes = s.as_bytes();
    let mut space_count = 0;
    let mut start_index = 0;
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            space_count += 1;
            
            if space_count == 1 {
                // Found end of first word
                start_index = i + 1;
            } else if space_count == 2 {
                // Found end of second word
                return Some(&s[start_index..i]);
            }
        }
    }
    
    // Handle case where string ends with second word
    if space_count == 1 && start_index < s.len() {
        return Some(&s[start_index..]);
    }
    
    None
}

// Function to find the first word in a string
// Takes a reference to a String (&String) as input
// Returns a string slice (&str) pointing to the first word
fn first_word(s: &String) -> &str {
    // Convert the string to bytes for byte-level inspection
    // Strings in Rust are UTF-8 encoded, so we work with bytes
    let bytes = s.as_bytes();  // Returns a slice of bytes (&[u8])

    // Iterate through the bytes with enumeration
    // enumerate() gives us (index, &byte) pairs
    for (i, &item) in bytes.iter().enumerate() {
        // Check if current byte is a space (ASCII value 32)
        // b' ' is byte literal for space character
        if item == b' ' {
            // Found a space! Return slice from start to space position
            // &s[0..i] creates a string slice from index 0 to i (exclusive)
            return &s[0..i];
        }
    }

    // If no space found, return the entire string as a slice
    // &s[..] is equivalent to &s[0..s.len()]
    &s[..]
}

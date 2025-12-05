let s1 = String::from("hello");
let s2 = s1;  // s1's data is MOVED to s2, s1 is now invalid

println!("{s1}, world!"); // ERROR: s1 is no longer valid here!

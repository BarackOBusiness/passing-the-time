// This code demonstrates the usage of slices, page 6.4, or:
// https://google.github.io/comprehensive-rust/basic-syntax/slices.html


pub fn main() {
    // Create a length 6 array, populated with intervals of 10
    let a: [i32; 6] = [10, 20, 30, 40, 50, 60];
    println!("a: {a:?}");

    // To create a slice, we borrow a and specify the starting and ending indices in the brackets
    let s: &[i32] = &a[2..4];
    // after this point, modifying a[2] or a[3] is not permitted for memory safety reasons, although both can be read
    println!("s: {s:?}");
}

// This code demonstrates string slices and strings, and it can be found on page 6.4.1, or:


pub fn string_slices() {
    // Create an immutable reference to a string slice
    let s1: &str = "World";
    println!("s1: {s1}");

    // Create a string buffer, which is mutable
    let mut s2: String = String::from("Hello ");
    println!("s2: {s2}");
    // We can push a string literal to a string buffer, as it is just a vec of UTF-8 chars
    s2.push_str(s1);
    println!("s2: {s2}");
    
    let s3: &str = &s2[6..];
    println!("s3: {s3}");
}
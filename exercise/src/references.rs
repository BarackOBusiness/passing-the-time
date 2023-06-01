// This code demonstrates the usage of references in use on a mutable value, page 6.3, or:
// https://google.github.io/comprehensive-rust/basic-syntax/references.html

// The sample code shown
pub fn main() {
    // Create an integer with a value of 10
    let mut x: i32 = 10;
    // create a reference to a mutable integer, x
    let ref_x: &mut i32 = &mut x;
    // Asterisk dereferences the variable ref_x and returns the data
    *ref_x = 20;
    // Now that the underlying memory for the value is overwritten, both ref_x and x would return 20.
    println!("x: {x}");
}

// Functions below this point may be my playground to test things
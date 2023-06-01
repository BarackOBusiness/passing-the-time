// Methods page, 6.5.2, otherwise visitable at https://google.github.io/comprehensive-rust/basic-syntax/methods.html

// Create a new struct, Rectangle, this has some data, width and height, which are unsigned integer.
struct Rectangle {
    width: u32,
    height: u32,
}

// The impl keyword declares an implementation for a type, which is much like a class
impl Rectangle {
    // Functions that are a part of a type's implementation are called methods, they are associated with a type
    // and may contain a self argument that is an instance of the associated type.
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // This method contains a reference to a mutable self, and as such does not need to return a value
    // it increments the width of the rectangle by an unsigned integer delta
    fn inc_width(&mut self, delta: u32) {
        self.width += delta;
    }
}

// A simple demonstration of some of this
pub fn main() {
    let mut rect = Rectangle { width: 10, height: 5 };
    println!("old area: {}", rect.area());
    rect.inc_width(5);
    println!("new area: {}", rect.area());
}
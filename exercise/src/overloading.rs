// Overloading page, 6.5.3, or https://google.github.io/comprehensive-rust/basic-syntax/functions-interlude.html
// as stated in the course, overloading is not supported, however we can emulate it via generics

// T is a generic, it accepts everything, so this method takes two of anything and returns one of anything
fn pick_one<T>(a: T, b: T) -> T {
    // If the process id is divisble by 2, we'll return a, otherwise, return b
    if std::process::id() % 2 == 0 { a } else { b }
}

pub fn main() {
    // This function is so versatile we can do both a coin flip:
    println!("coin toss: {}", pick_one("heads", "tails"));
    // WTF
    println!("cash prize: {}", pick_one(500, 1000));
}
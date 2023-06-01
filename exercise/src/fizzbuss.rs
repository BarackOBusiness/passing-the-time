// Fizz buzz, written in rust, taken straight from the course, while I don't necessarily need an explanation to understand this stuff, I'm just going to insert this anyway.
// Besides, I created the file before reasoning that out, and I accidentally named it fizz buss and thought it was the funniest shit,
// couldn't bring myself to get rid of it after that, so why not give it some purpose

#[allow(dead_code)]
pub fn main() {
    print_fizzbuzz_to(30);
}

// Checks if number n is divisible by another number divisor.
fn is_divisible(n: u32, divisor: u32) -> bool {
    // Obviously check if it's 0 first, as that's always a false
    if divisor == 0 {
        return false;
    }
    // Return the value of this boolean expression
    n % divisor == 0
}

// Checks if the number should be fizz or buss
fn fizzbuzz(n: u32) -> String {
    // First the fizz condition, whether the number is divisble by 3
    let fizz = if is_divisible(n, 3) { "fizz" } else { "" };
    // And the buzz condition, whether the number is divisible by 5
    let buzz = if is_divisible(n, 5) { "buzz" } else { "" };
    // Should both conditions be false, we need to return just the number, so we do that here
    if fizz.is_empty() && buzz.is_empty() {
        return format!("{n}");
    }
    // the format!() macro can construct a String with the power of formatting various variables into it, either one of the conditions are true or both are,
    // if this segment of the code runs, so we can print fizz, buzz, or fizzbuzz, as expected.
    format!("{fizz}{buzz}")
}

// An abstraction to a for loop, which runs fizz buzz across a series of numbers
fn print_fizzbuzz_to(n: u32) {
    // Makes use of a range in the for loop, this iterates over every number in 1 and 20, inclusive
    for i in 1..=n {
        println!("{}", fizzbuzz(i));
    }
}
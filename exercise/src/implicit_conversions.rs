// Original code from 7.1

// fn multiply(x: i16, y: i16) -> i16 {
//     x * y
// }

// fn main() {
//     let x: i8 = 15;
//     let y: i16 = 1000;

//     println!("{x} * {y} = {}", multiply(x, y));
// }

// Fixed version
fn multiply(x: i16, y: i16) -> i16 {
    x * y
}

pub fn main() {
    let x: i8 = 15;
    let y: i16 = 1000;

    // We use the into method
    println!("{x} * {y} = {}", multiply(x.into(), y));
}
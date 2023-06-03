mod bit;
mod packed_bits;

use bit::Bit;
use packed_bits::Bitx8;

use std::vec;

fn main() {
    println!("Hello, world!");

    let mut zero: Bit = 0.into();

    println!("Bit is currently: {}", zero);

    let one: &mut Bit = &mut zero;
    
    one.flip();

    println!("Bit is now: {}", zero);

    // TODO: Make this workflow work and preferably in a less painful way, if you can call it a "workflow" considering it's just a bunch of bits
    let bits = vec![Bit::Zero, Bit::One, Bit::Zero, Bit::Zero, Bit::One, Bit::One, Bit::Zero, Bit::One];

    let bit_arr: Box<[Bit]> = bits.into_boxed_slice();

    let bit_arr = *bit_arr;

    let byte: Bitx8 = Bitx8::new(bit_arr);

    let byte = byte.unpack();
}

use super::bit::*;

pub struct Bitx8 {
    packed_bits: u8
}

impl Bitx8 {
    pub fn new(bits: [Bit; 8]) -> Self {
        let mut bin_int: String = String::new();
        
        for b in bits {
            match b {
                Bit::Zero => bin_int.push_str("0"),
                Bit::One => bin_int.push_str("1"),
            }
        }

        Bitx8 { 
            packed_bits: u8::from_str_radix(bin_int.as_str(), 2).unwrap()
        }
    }

    pub fn unpack(self) -> [Bit; 8] {
        let bin_int: String = format!("{:b}", self.packed_bits);

        let mut bit_arr: [Bit; 8];

        for (i, c) in bin_int.chars().enumerate() {
            bit_arr[i] = match c {
                '0' => Bit::Zero,
                '1' => Bit::One,
                _ => panic!("Intermediary strings within a pack or unpack operation should not contain characters other than 1 or 0"),
            }
        }

        bit_arr 
    }    
}
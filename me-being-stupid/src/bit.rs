use core::fmt::Display;

// #[derive(Clone, Copy)]
pub enum Bit {
    Zero,
    One
}

impl Bit {
    pub fn new() -> Self {
        Bit::Zero
    }

    pub fn flip(&mut self) {
        *self = match self {
            Bit::Zero => Bit::One,
            Bit::One => Bit::Zero
        }
    }
}

impl Display for Bit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Bit::Zero => write!(f, "0"),
            Bit::One => write!(f, "1"),
        }
    }
}

impl Into<bool> for Bit {
    fn into(self) -> bool {
        match self {
            Bit::Zero => false,
            Bit::One => true,
        }
    }
}

impl Into<Bit> for bool {
    fn into(self) -> Bit {
        match self {
            false => Bit::Zero,
            true => Bit::One
        }
    }
}

impl From<u8> for Bit {
    fn from(value: u8) -> Self {
        match value {
            0 => Bit::Zero,
            _ => Bit::One,
        }
    }
}

impl From<Bit> for u8 {
    fn from(value: Bit) -> Self {
        match value {
            Bit::Zero => 0,
            Bit::One => 1
        }
    }
}
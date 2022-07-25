#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Opcode(u16);

impl Opcode {
    pub const fn first(self) -> usize {
        (self.0 >> 12) as usize
    }
    pub const fn last(self) -> usize {
        (self.0 << 12) as usize
    }

    pub const fn last_two(self) -> u8 {
        (self.0 & 0x00FF) as u8
    }

    pub const fn x(self) -> usize {
        ((self.0 & 0x0F00) >> 8) as usize
    }

    pub const fn y(self) -> usize {
        ((self.0 & 0x00F0) >> 4) as usize
    }

    pub const fn n(self) -> usize {
        (self.0 & 0x000F) as usize
    }

    pub const fn nn(self) -> u8 {
        (self.0 & 0x00FF) as u8
    }

    pub const fn nnn(self) -> u16 {
        self.0 & 0x0FFF
    }

    pub const fn as_u16(self) -> u16 {
        self.0 as u16
    }
}

impl From<(u8, u8)> for Opcode {
    fn from((a, b): (u8, u8)) -> Self {
        Opcode((a as u16) << 8 | b as u16)
    }
}

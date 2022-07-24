#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Opcode(u16);

impl Opcode {
    pub fn first(&self) -> usize {
        (self.0 >> 12) as usize
    }
    pub fn last(&self) -> usize {
        (self.0 << 12) as usize
    }

    pub fn last_two(&self) -> u8 {
        (self.0 & 0x00FF) as u8
    }

    pub fn x(&self) -> usize {
        ((self.0 & 0x0F00) >> 8) as usize
    }

    pub fn y(&self) -> usize {
        ((self.0 & 0x00F0) >> 4) as usize
    }

    pub fn n(&self) -> usize {
        (self.0 & 0x000F) as usize
    }

    pub fn nn(&self) -> u8 {
        (self.0 & 0x00FF) as u8
    }

    pub fn nnn(&self) -> u16 {
        self.0 & 0x0FFF
    }

    pub fn as_u16(&self) -> u16 {
        self.0 as u16
    }
}

impl From<(u8, u8)> for Opcode {
    fn from((a, b): (u8, u8)) -> Self {
        Opcode((a as u16) << 8 | b as u16)
    }
}

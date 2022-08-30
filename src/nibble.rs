use crate::{
    address::Address,
    register::{Register, X, Y},
};

pub(crate) trait NibbleExt: Sized + std::ops::BitAnd<u16, Output = u16> {
    fn n(self) -> u8 {
        (self & 0x000F) as u8
    }
    /// Gets the 2nd and 3rd nibble of the opcode.
    fn nn(self) -> u8 {
        (self & 0x00FF_u16) as u8
    }
    /// Gets the 2nd, 3rd, and 4th nibble of the opcode.
    fn nnn(self) -> Address {
        (self & 0x0FFF_u16).into()
    }
    /// Gets the 2nd nibble of the opcode.
    fn x(self) -> Register<X> {
        ((self & 0x0F00_u16) >> 8).into()
    }
    /// Gets the 3rd nibble of the opcode.
    fn y(self) -> Register<Y> {
        ((self & 0x00F0_u16) >> 4).into()
    }
}

impl NibbleExt for u16 {}

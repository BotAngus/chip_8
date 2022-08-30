use std::fmt::Display;

use crate::{
    address::Address,
    nibble::NibbleExt,
    register::{Register, X, Y},
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UnknownOpcode(u16);

impl Display for UnknownOpcode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Opcode not recognised: {:#06x}", self.0)
    }
}

impl std::error::Error for UnknownOpcode {}

#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Opcode {
    Cls,
    Ret,
    JpAddress(Address),
    CallAddress(Address),
    SeVxByte(Register<X>, u8),
    SneVxByte(Register<X>, u8),
    SeVxVy(Register<X>, Register<Y>),
    LdVxByte(Register<X>, u8),
    AddVxByte(Register<X>, u8),
    LdVxVy(Register<X>, Register<Y>),
    OrVxVy(Register<X>, Register<Y>),
    AndVxVy(Register<X>, Register<Y>),
    XorVxVy(Register<X>, Register<Y>),
    AddVxVy(Register<X>, Register<Y>),
    SubVxVy(Register<X>, Register<Y>),
    ShrVxVy(Register<X>, Register<Y>),
    SubnVxVy(Register<X>, Register<Y>),
    ShlVxVy(Register<X>, Register<Y>),
    SneVxVy(Register<X>, Register<Y>),
    LdIAddress(Address),
    JpV0Address(Address),
    RndVxByte(Register<X>, u8),
    DrwVxVyNibble(Register<X>, Register<Y>, u8),
    SkpVx(Register<X>),
    SknpVx(Register<X>),
    LdVxDt(Register<X>),
    LdVxK(Register<X>),
    LdDtVx(Register<X>),
    LdStVx(Register<X>),
    AddIVx(Register<X>),
    LdFVx(Register<X>),
    LdBVx(Register<X>),
    LdIVx(Register<X>),
    LdVxI(Register<X>),
}

impl TryFrom<(u8, u8)> for Opcode {
    type Error = UnknownOpcode;
    fn try_from((a, b): (u8, u8)) -> Result<Self, Self::Error> {
        use Opcode::*;
        let code = (a as u16) << 8 | b as u16;

        Ok(match code & 0xF000 {
            0x0000 => match code & 0x00FF {
                0x00E0 => Cls,
                0x00EE => Ret,
                _ => return Err(UnknownOpcode(code)),
            },

            0x1000 => JpAddress(code.nnn()),
            0x2000 => CallAddress(code.nnn()),
            0x3000 => SeVxByte(code.x(), code.nn()),
            0x4000 => SneVxByte(code.x(), code.nn()),
            0x5000 => SeVxVy(code.x(), code.y()),
            0x6000 => LdVxByte(code.x(), code.nn()),
            0x7000 => AddVxByte(code.x(), code.nn()),
            0x8000 => match code & 0xF00F {
                0x8000 => LdVxVy(code.x(), code.y()),
                0x8001 => OrVxVy(code.x(), code.y()),
                0x8002 => AndVxVy(code.x(), code.y()),
                0x8003 => XorVxVy(code.x(), code.y()),
                0x8004 => AddVxVy(code.x(), code.y()),
                0x8005 => SubVxVy(code.x(), code.y()),
                0x8006 => ShrVxVy(code.x(), code.y()),
                0x8007 => SubnVxVy(code.x(), code.y()),
                0x800E => ShlVxVy(code.x(), code.y()),
                _ => return Err(UnknownOpcode(code)),
            },
            0x9000 => SneVxVy(code.x(), code.y()),
            0xA000 => LdIAddress(code.nnn()),
            0xB000 => JpV0Address(code.nnn()),
            0xC000 => RndVxByte(code.x(), code.nn()),
            0xD000 => DrwVxVyNibble(code.x(), code.y(), code.n()),
            0xE000 => match code & 0xF0FF {
                0xE09E => SkpVx(code.x()),
                0xE0A1 => SknpVx(code.x()),
                _ => return Err(UnknownOpcode(code)),
            },
            0xF000 => match code & 0xF0FF {
                0xF007 => LdVxDt(code.x()),
                0xF00A => LdVxK(code.x()),
                0xF015 => LdDtVx(code.x()),
                0xF018 => LdStVx(code.x()),
                0xF01E => AddIVx(code.x()),
                0xF029 => LdFVx(code.x()),
                0xF033 => LdBVx(code.x()),
                0xF055 => LdIVx(code.x()),
                0xF065 => LdVxI(code.x()),
                _ => return Err(UnknownOpcode(code)),
            },
            _ => return Err(UnknownOpcode(code)),
        })
    }
}

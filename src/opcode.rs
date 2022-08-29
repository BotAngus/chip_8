use std::fmt::Display;

use crate::{
    nibble::NibbleExt,
    types::{Address, Register, X, Y},
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum OpcodeData {
    Empty,
    X(Register<X>),
    Y(Register<Y>),
    NNN(Address),
    XYN(Register<X>, Register<Y>, u8),
    XY(Register<X>, Register<Y>),
    NN(u8),
    XNN(Register<X>, u8),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum OpcodeName {
    /// 0E00
    Cls,

    /// 00EE
    Ret,

    /// 1NNN
    Jp,

    /// BNNN
    JpV0,

    /// 2NNN
    Call,

    /// 3XNN, 5XY0
    Se,

    /// 4XNN, 9XY0
    Sne,

    /// 6XNN, 8XY0, ANNN
    Ld,

    /// 7XNN, 8XY4
    Add,

    /// 8XY1
    Or,

    /// 8XY2
    And,

    /// 8XY3
    Xor,

    /// 8XY5
    Sub,

    /// 8XY6
    Shr,

    /// 8XY7
    SubN,

    /// 8XYE
    Shl,

    /// CXNN
    Rnd,

    /// DXYN
    Drw,

    /// EX9E
    Skp,

    /// EXA1
    Sknp,
}
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Opcode {
    name: OpcodeName,
    data: OpcodeData,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UnknownOpcode(u16);

impl Display for UnknownOpcode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Opcode not recognised: {:#06x}", self.0)
    }
}

impl std::error::Error for UnknownOpcode {}

impl TryFrom<(u8, u8)> for Opcode {
    type Error = UnknownOpcode;
    fn try_from((a, b): (u8, u8)) -> Result<Self, Self::Error> {
        use OpcodeData::*;
        use OpcodeName::*;
        let code = (a as u16) << 8 | b as u16;

        Ok(match code & 0xF000 {
            0x0000 => match code {
                0x0E00 => Self {
                    name: Cls,
                    data: Empty,
                },
                0x00EE => Self {
                    name: Ret,
                    data: Empty,
                },
                _ => return Err(UnknownOpcode(code)),
            },
            0x1000 => Self {
                name: Jp,
                data: NNN(code.nnn()),
            },
            0x2000 => Self {
                name: Call,
                data: NNN(code.nnn()),
            },

            0x3000 => Self {
                name: Se,
                data: XNN(code.x(), code.nn()),
            },
            0x4000 => Self {
                name: Sne,
                data: XNN(code.x(), code.nn()),
            },
            0x5000 => Self {
                name: Se,
                data: XY(code.x(), code.y()),
            },
            0x6000 => Self {
                name: Ld,
                data: XNN(code.x(), code.nn()),
            },
            0x7000 => Self {
                name: Add,
                data: XNN(code.x(), code.nn()),
            },
            0x8000 => match code & 0x000F {
                0x0000 => Self {
                    name: Ld,
                    data: XY(code.x(), code.y()),
                },
                0x0001 => Self {
                    name: Or,
                    data: XY(code.x(), code.y()),
                },
                0x0002 => Self {
                    name: And,
                    data: XY(code.x(), code.y()),
                },
                0x0003 => Self {
                    name: Xor,
                    data: XY(code.x(), code.y()),
                },

                0x0004 => Self {
                    name: Add,
                    data: XY(code.x(), code.y()),
                },

                0x0005 => Self {
                    name: Sub,
                    data: XY(code.x(), code.y()),
                },

                0x0006 => Self {
                    name: Shr,
                    data: XY(code.x(), code.y()),
                },

                0x0007 => Self {
                    name: SubN,
                    data: XY(code.x(), code.y()),
                },

                0x000E => Self {
                    name: Shl,
                    data: XY(code.x(), code.y()),
                },
                _ => return Err(UnknownOpcode(code)),
            },

            0x9000 => Self {
                name: Sne,
                data: XY(code.x(), code.y()),
            },

            0xA000 => Self {
                name: Ld,
                data: NNN(code.nnn()),
            },
            0xB000 => Self {
                name: JpV0,
                data: NNN(code.nnn()),
            },

            0xC000 => Self {
                name: Rnd,
                data: XNN(code.x(), code.nn()),
            },
            0xD000 => Self {
                name: Drw,
                data: XYN(code.x(), code.y(), code.n()),
            },

            0xE000 => match code & 0x00FF {
                0x009E => Self {
                    name: Skp,
                    data: X(code.x()),
                },
                0x00A1 => Self {
                    name: Sknp,
                    data: X(code.x()),
                },

                _ => return Err(UnknownOpcode(code)),
            },

            _ => return Err(UnknownOpcode(code)),
        })
    }
}

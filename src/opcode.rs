// http://devernay.free.fr/hacks/chip8/C8TECH10.HTM
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[non_exhaustive]
#[allow(non_camel_case_types)]
pub enum Opcode {
    /// 00E0 Clear the display.
    CLS,

    /// 00EE Return from a subroutine.
    RET,

    /// 1NNN The interpreter sets the program counter to nnn
    JP(usize),

    /// 2NNN Call subroutine at nnn.
    CALL(usize),

    /// 3XKK Skip next instruction if Vx = kk.
    SE(usize, u8),

    /// 4XKK Skip next instruction if Vx != kk.
    SNE(usize, u8),
    /// 5XY0 Skip next instruction if Vx = Vy.
    SE_REG(usize, usize),

    /// 6XKK Set Vx = kk.
    LD(usize, u8),

    /// 7XKK Set Vx = Vx + kk.
    ADD(usize, u8),

    /// 8XY0 Set Vx = Vy.
    LD_REG(usize, usize),

    /// 8XY1 Set Vx = Vx | Vy.
    OR(usize, usize),

    /// 8XY2 Set Vx = Vx & Vy.
    AND(usize, usize),

    /// 8XY3 Set Vx = Vx ^ Vy.
    XOR(usize, usize),

    /// 8XY4 Set Vx = Vx + Vy, set VF = carry.
    ADD_REG(usize, usize),

    /// 8XY5 Set Vx = Vx - Vy, set VF = NOT borrow.
    SUB(usize, usize),

    /// 8XY6 Set Vx = Vx SHR 1.
    SHR(usize, usize),
    
    /// 8XY7 Set Vx = Vy - Vx, set VF = NOT borrow.
    SUB_REG(usize, usize),

    /// 8XYE Set Vx = Vx SHL 1.
    SHL(usize, usize),

    /// 9XY0 Skip next instruction if Vx != Vy.
    SNE_REG(usize, usize),

    /// ANNN Set I = nnn.
    LD_I(usize),

    /// BNNN Jump to location nnn + V0.
    JP_V0(usize),

    /// CXKK Set Vx = random byte & kk.
    RND(usize, u8),

    /// DXYN Draw a sprite at coordinate (Vx, Vy), with width 8 pixels and height n pixels.
    DRW(usize, usize, u8),

    /// EX9E Skip next instruction if key with the value of Vx is pressed.
    SKP(usize),

    /// EXA1 Skip next instruction if key with the value of Vx is not pressed.
    SKNP(usize),

    /// FX07 Set Vx = delay timer value.
    LD_DELAY(usize),

    /// FX0A Wait for a key press, store the value of the key in Vx.
    LD_KEY(usize),

    /// FX15 Set delay timer = Vx.
    LD_DELAY_REG(usize),

    /// FX18 Set sound timer = Vx.
    LD_SOUND_REG(usize),

    /// FX1E Set I = I + Vx.
    ADD_I(usize),

    /// FX29 Set I = location of sprite for digit Vx.
    LD_FONT(usize),

    /// FX33 Set I = location of BCD representation of Vx.
    LD_BCD(usize),

    /// FX55 Store registers V0 through Vx in memory starting at location I.
    LD_REG_I(usize),

    /// FX65 Read registers V0 through Vx from memory starting at location I.
    LD_I_REG(usize),
    
    /// 0NNN Unknown opcode.
    UNKNOWN(usize),
}

impl Opcode {
    pub fn decode(tuple: (u8, u8)) -> Opcode {
        let as_u16 = (tuple.0 as u16) << 8 | tuple.1 as u16;
        Opcode::from(as_u16)
    }

    /// Gets the 4th nibble of the opcode.
    const fn n(code: u16) -> u8 {
        (code & 0x000F) as u8
    }
    /// Gets the 2nd and 3rd nibble of the opcode.
    const fn kk(code: u16) -> u8 {
        (code & 0x00FF) as u8
    }
    /// Gets the 2nd, 3rd, and 4th nibble of the opcode.
    const fn nnn(code: u16) -> usize {
        (code & 0x0FFF) as usize
    }
    /// Gets the 2nd nibble of the opcode.
    const fn x(code: u16) -> usize {
        ((code & 0x0F00) >> 8) as usize
    }
    /// Gets the 3rd nibble of the opcode.
    const fn y(code: u16) -> usize {
        ((code & 0x00F0) >> 4) as usize
    }
}

impl From<u16> for Opcode {
    fn from(code: u16) -> Self {
        match code {
            0x0000 => Opcode::CLS,
            0x000E => Opcode::RET,
            0x1000 => Opcode::JP(Opcode::nnn(code)),
            0x2000 => Opcode::CALL(Opcode::nnn(code)),
            0x3000 => Opcode::SE(Opcode::x(code), Opcode::kk(code)),
            0x4000 => Opcode::SNE(Opcode::x(code), Opcode::kk(code)),
            0x5000 => Opcode::SE_REG(Opcode::x(code), Opcode::y(code)),
            0x6000 => Opcode::LD(Opcode::x(code), Opcode::kk(code)),
            0x7000 => Opcode::ADD(Opcode::x(code), Opcode::kk(code)),
            0x8000 => match code & 0x000F {
                0x0000 => Opcode::LD_REG(Opcode::x(code), Opcode::y(code)),
                0x0001 => Opcode::OR(Opcode::x(code), Opcode::y(code)),
                0x0002 => Opcode::AND(Opcode::x(code), Opcode::y(code)),
                0x0003 => Opcode::XOR(Opcode::x(code), Opcode::y(code)),
                0x0004 => Opcode::ADD_REG(Opcode::x(code), Opcode::y(code)),
                0x0005 => Opcode::SUB(Opcode::x(code), Opcode::y(code)),
                0x0006 => Opcode::SHR(Opcode::x(code), Opcode::y(code)),
                0x0007 => Opcode::SUB_REG(Opcode::x(code), Opcode::y(code)),
                0x000E => Opcode::SHL(Opcode::x(code), Opcode::y(code)),
                _ => Opcode::UNKNOWN(code as usize),
            },
            0x9000 => Opcode::SNE_REG(Opcode::x(code), Opcode::y(code)),
            0xA000 => Opcode::LD_I(Opcode::nnn(code)),
            0xB000 => Opcode::JP_V0(Opcode::nnn(code)),
            0xC000 => Opcode::RND(Opcode::x(code), Opcode::kk(code)),
            0xD000 => Opcode::DRW(Opcode::x(code), Opcode::y(code), Opcode::n(code)),
            0xE000 => match code & 0x00FF {
                0x009E => Opcode::SKP(Opcode::x(code)),
                0x00A1 => Opcode::SKNP(Opcode::x(code)),
                _ => Opcode::UNKNOWN(code as usize),
            },
            0xF000 => match code & 0x00FF {
                0x0007 => Opcode::LD_DELAY(Opcode::x(code)),
                0x000A => Opcode::LD_KEY(Opcode::x(code)),
                0x0015 => Opcode::LD_DELAY_REG(Opcode::x(code)),
                0x0018 => Opcode::LD_SOUND_REG(Opcode::x(code)),
                0x001E => Opcode::ADD_I(Opcode::x(code)),
                0x0029 => Opcode::LD_FONT(Opcode::x(code)),
                0x0033 => Opcode::LD_BCD(Opcode::x(code)),
                0x0055 => Opcode::LD_REG_I(Opcode::x(code)),
                0x0065 => Opcode::LD_I_REG(Opcode::x(code)),
                _ => Opcode::UNKNOWN(code as usize),
            },
            _ => Opcode::UNKNOWN(code as usize),
        }
    }
}

pub type Address = usize;
pub type Register = usize;
pub type Byte = u8;
// http://devernay.free.fr/hacks/chip8/C8TECH10.HTM
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[non_exhaustive]
#[allow(non_camel_case_types)]
pub enum Opcode {
    /// 0NNN Jump to a machine code routine at nnn
    SYS_NNN(Address),

    /// 00E0 Clear the display
    CLS,

    /// 00EE Return from a subroutine
    /// The interpreter sets the program counter to the address at the top of the stack, then subtracts 1 from the stack pointer
    RET,

    /// 1NNN Jump to location nnn
    /// The interpreter sets the program counter to nnn
    JP_NNN(Address),

    /// 2NNN Call subroutine at nnn
    /// The interpreter increments sthe stack pointer, then puts the current PC on the top of the stack. The PC is then set to nnn
    CALL_NNN(Address),

    /// 3XNN Skip next instruction if Vx = kk
    /// The interpreter compares register Vx to kk, and if they are equal, increments the program counter by 2.
    SE_XNN(Register),

    /// 4XNN Skip next instruction if Vx != kk
    /// The interpreter compares register Vx to kk, and if they are not equal, increments the program counter by 2.
    SNE_XNN(Register, Byte),

    /// 5XY0 Skip next instruction if Vx = Vy
    /// The interpreter compares register Vx to register Vy, and if they are equal, increments the program counter by 2.
    SE_XY(Register, Register),

    /// 6XNN Set Vx = kk
    /// The interpreter puts the value kk into register Vx.
    LD_XNN(Register, Byte),

    /// 7XNN Set Vx = Vx + kk
    /// Adds the value kk to the value of register Vx, then stores the result in Vx.
    ADD_XNN(Register, Byte),

    /// 8XY0 Set Vx = Vy
    /// Stores the value of register Vy in register Vx.
    LD_XY(Register, Register),

    /// 8XY1 Set Vx = Vx OR Vy
    /// Performs a bitwise OR on the values of Vx and Vy
    OR_XY(Register, Register),

    /// 8XY2 Set Vx = Vx AND Vy
    /// Performs a bitwise AND on the values of Vx and Vy
    AND_XY(Register, Register),

    /// 8XY3 Set Vx = Vx XOR Vy
    /// Performs a bitwise XOR on the values of Vx and Vy
    XOR_XY(Register, Register),

    /// 8XY4 Adds the value of Vy to the value of Vx, then stores the result in Vx. Vf is set to 1 when there's a carry, and to 0 when there isn't.
    ADD_XY(Register, Register),

    /// 8XY5 Vx = Vx - Vy, Vf = NOT borrow
    /// If Vx > Vy, then Vf is set to 1, otherwise 0. Then Vy is subtracted from Vx, and the results stored in Vx.
    SUB_XY(Register, Register),

    /// 8XY6 Stores the least significant bit of Vy in Vf and shifts Vy right by 1. Vx = Vy >> 1
    SHR_XY(Register, Register),

    /// 8XY7 Vx = Vy - Vx, Vf = NOT borrow
    /// If Vy > Vx, then Vf is set to 1, otherwise 0. Then Vx is subtracted from Vy, and the results stored in Vx.
    SUBN_XY(Register, Register),

    /// 8XYE Stores the most significant bit of Vy in Vf and shifts Vy left by 1. Vx = Vy << 1
    SHL_XY(Register, Register),

    /// 9XY0 Skip next instruction if Vx != Vy
    /// The interpreter compares register Vx to register Vy, and if they are not equal, increments the program counter by 2.
    SNE_XY(Register, Register),

    /// ANNN Set I = nnn
    /// The interpreter sets the address register I to nnn.
    LD_I_NNN(Address),

    /// BNNN Jump to location nnn + V0
    /// The interpreter sets the program counter to nnn plus the value of V0
    JP_V0_NNN(Address),

    /// CXNN Set Vx = random byte AND kk
    /// The interpreter generates a random number from 0 to 255, which is then ANDed with the value kk. The results are stored in register Vx.
    RND_XNN(Register, Byte),

    /// DXYN Display n-byte sprite starting at memory location I at (Vx, Vy), set Vf = collision
    /// The interpreter reads n bytes from memory, starting at the address stored in I. These bytes are then displayed as sprites on screen at coordinates (Vx, Vy). Sprites are 8 pixels wide and 8 pixels tall. If an intersection is detected, Vf is set to 1, otherwise it is
    DRW_XYN(Register, Register, Byte),

    /// EX9E Skip next instruction if key with the value of Vx is pressed
    /// Checks the keyboard, and if the key stored in Vx is currently pressed, increments the program counter by 2.
    SKP_X(Register),

    /// EXA1 Skip next instruction if key with the value of Vx is not pressed
    /// Checks the keyboard, and if the key stored in Vx is currently not pressed, increments the program counter by 2.
    SKNP_X(Register),

    /// FX07 Set Vx = delay timer value
    /// The interpreter copies the value of DT to Vx.
    LD_X_DT(Register),

    /// FX0A Wait for a key press, store the value of the key in Vx
    /// The interpreter sets the value of Vx to the value of the key pressed.
    LD_X_KEY(Register),

    /// FX15 Set delay timer = Vx
    LD_DT_X(Register),
    
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
        match code & 0xF000 {
            0x0 => match code & 0x00FF {
                0xE0 => Opcode::CLS,
                0xEE => Opcode::RET,
                _ => Opcode::UNKNOWN(code),
            },

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
                _ => Opcode::UNKNOWN(code),
            },
            0x9000 => Opcode::SNE_REG(Opcode::x(code), Opcode::y(code)),
            0xA000 => Opcode::LD_I(Opcode::nnn(code)),
            0xB000 => Opcode::JP_V0(Opcode::nnn(code)),
            0xC000 => Opcode::RND(Opcode::x(code), Opcode::kk(code)),
            0xD000 => Opcode::DRW(Opcode::x(code), Opcode::y(code), Opcode::n(code)),
            0xE000 => match code & 0x00FF {
                0x009E => Opcode::SKP(Opcode::x(code)),
                0x00A1 => Opcode::SKNP(Opcode::x(code)),
                _ => Opcode::UNKNOWN(code),
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
                _ => Opcode::UNKNOWN(code),
            },
            _ => Opcode::UNKNOWN(code),
        }
    }
}

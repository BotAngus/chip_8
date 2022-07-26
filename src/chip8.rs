use std::time::SystemTime;

pub struct Chip8<const R: usize = 4096, const X: usize = 128, const Y: usize = 64> {
    /// Default 4 kilobytes of RAM
    memory: [u8; R],
    /// Points to current instruction
    pc: usize,

    /// 64 x 32 pixels, monochrome
    display: [[u8; Y]; X],
    /// Index register
    i_reg: usize,
    /// Variable registers
    v_reg: [u8; 16],
    /// 12/16-bit stack
    stack: Stack,
}

#[macro_export]
macro_rules! new {
    ($R:expr, $X:expr, $Y:expr) => {
        chip8::Chip8::<$R, $X, $Y>::new()
    };
}
pub(crate) use new;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct State<const X: usize, const Y: usize> {
    display_update: bool,
    display: Option<[[u8; Y]; X]>,
}

use crate::{opcode::Opcode, stack::Stack};
#[allow(dead_code)]
impl<const R: usize, const X: usize, const Y: usize> Chip8<R, X, Y> {
    pub fn new() -> Chip8<R, X, Y> {
        let mut chip8 = Chip8::<R, X, Y> {
            memory: [0; R],
            pc: 0x200,

            display: [[0; Y]; X],
            i_reg: 0,
            v_reg: [0; 16],
            stack: Stack::<32>::new(),
        };

        chip8.load_font();
        chip8
    }

    fn load_font(&mut self) {
        self.memory[0x50..=0x9F].copy_from_slice(&Chip8::<R, X, Y>::FONT);
    }

    pub fn load_program<'a, T: Into<&'a [u8]>>(&mut self, input: T) {
        let input: &[u8] = input.into();
        self.memory[0x200..(0x200 + input.len())].copy_from_slice(input);
    }

    fn fetch(&mut self) -> Opcode {
        // Bitwise shift the first part of the instruction by 8 and then bitwise OR it with the second part
        let opcode = Opcode::decode((self.memory[self.pc], self.memory[self.pc + 1]));

        self.pc += 2;
        opcode
    }

    pub fn tick(&mut self) {}
    fn decode_and_execute(&mut self, opcode: Opcode) {
        match opcode {
            Opcode::CLS => self.display = [[0; Y]; X],
            Opcode::RET => self.pc = self.stack.pop(),
            Opcode::JP(addr) => self.pc = addr,
            Opcode::CALL(addr) => {
                self.stack.push(self.pc);
                self.pc = addr;
            }
            Opcode::SE(x, y) => {
                if self.v_reg[x] == y {
                    self.pc += 2;
                }
            }
            Opcode::SNE(x, y) => {
                if self.v_reg[x] != y {
                    self.pc += 2;
                }
            }
            Opcode::LD(x, y) => self.v_reg[x] = y,
            Opcode::ADD(x, y) => {
                self.v_reg[x] = self.v_reg[x].wrapping_add(y);
            }
            Opcode::OR(x, y) => self.v_reg[x] |= self.v_reg[y],
            Opcode::AND(x, y) => self.v_reg[x] &= self.v_reg[y],
            Opcode::XOR(x, y) => self.v_reg[x] ^= self.v_reg[y],
            Opcode::ADD_REG(x, y) => {
                let (value, underflow) = self.v_reg[x].overflowing_add(self.v_reg[y]);

                self.v_reg[x] = value;
                self.v_reg[0x0F] = u8::from(underflow);
            }
            Opcode::SUB(x, y) => {
                let (value, underflow) = self.v_reg[x].overflowing_sub(self.v_reg[y]);

                self.v_reg[x] = value;
                self.v_reg[0x0F] = u8::from(underflow);
            }
            Opcode::SHR(x, _) => {
                self.v_reg[0x0F] = self.v_reg[x] & 1; // get least significant bit
                self.v_reg[x] >>= 1;
            }
            Opcode::SUB_REG(x, y) => {
                let (value, underflow) = self.v_reg[y].overflowing_sub(self.v_reg[x]);

                self.v_reg[x] = value;
                self.v_reg[0x0F] = u8::from(underflow);
            }
            Opcode::SHL(x, _) => {
                self.v_reg[0x0F] = self.v_reg[x] & 0x80; // get most significant bit
                self.v_reg[x] <<= 1;
            }
            Opcode::RND(x, y) => {
                self.v_reg[x] = SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .unwrap()
                    .as_nanos() as u8
                    & y
            }
            Opcode::DRW(x, y, n) => {
                let x = self.v_reg[x] as usize;
                let y = self.v_reg[y] as usize;
                let n = self.v_reg[n as usize] as usize;

                let mut collision = false;
                for i in 0..n {
                    let sprite = self.memory[self.i_reg as usize + i];
                    for j in 0..8 {
                        let bit = (sprite >> (7 - j)) & 1;
                        let x = (x + j) % X;
                        let y = (y + i) % Y;
                        if bit == 1 {
                            if self.display[x][y] == 1 {
                                collision = true;
                            }
                            self.display[x][y] ^= 1;
                        }
                    }
                }
                self.v_reg[0xF] = u8::from(collision);
            }

            Opcode::SE_REG(x, y) => {
                if self.v_reg[x] == self.v_reg[y] {
                    self.pc += 2;
                }
            }
            Opcode::LD_REG(x, y) => {
                self.v_reg[x] = self.v_reg[y];
            }
            Opcode::SNE_REG(x, y) => {
                if self.v_reg[x] != self.v_reg[y] {
                    self.pc += 2;
                }
            }
            Opcode::LD_I(addr) => self.i_reg = addr,
            Opcode::JP_V0(addr) => self.pc = addr + self.v_reg[0] as usize,

            Opcode::ADD_I(x) => self.i_reg += self.v_reg[x] as usize,
            Opcode::LD_FONT(_) => todo!(),
            Opcode::LD_BCD(_) => todo!(),
            Opcode::LD_REG_I(_) => todo!(),
            Opcode::LD_I_REG(_) => todo!(),
            Opcode::UNKNOWN(_) => todo!(),
            _ => panic!(),
        }
    }

    const FONT: [u8; 80] = [
        0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
        0x20, 0x60, 0x20, 0x20, 0x70, // 1
        0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
        0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
        0x90, 0x90, 0xF0, 0x10, 0x10, // 4
        0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
        0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
        0xF0, 0x10, 0x20, 0x40, 0x40, // 7
        0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
        0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
        0xF0, 0x90, 0xF0, 0x90, 0x90, // A
        0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
        0xF0, 0x80, 0x80, 0x80, 0xF0, // C
        0xE0, 0x90, 0x90, 0x90, 0xE0, // D
        0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
        0xF0, 0x80, 0xF0, 0x80, 0x80, // F
    ];
}

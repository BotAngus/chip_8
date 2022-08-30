use std::time::SystemTime;

use sdl2::sys::Screen;

use crate::{
    address::Address,
    memory::Memory,
    opcode::Opcode,
    register::{Register, Registers, I, T, X, Y},
    screen::Display,
    stack::Stack,
};

pub struct Chip8 {
    memory: Memory,
    pc: Address,
    i: Register<I>,
    stack: Stack,
    v: Registers,
}

impl Chip8 {
    const F: Register<T> = Register::<T>::from(0xF_usize);
    pub fn new() -> Self {
        let mut chip = Self {
            memory: Memory::new(),
            pc: 0x200_usize.into(), // 512
            i: 0_usize.into(),
            stack: Stack::new(),
            v: Registers::new(),
        };
        chip.load_font();
        chip
    }

    pub fn load_program<'a, T: Into<&'a [u8]>>(&mut self, program: T) {
        let input: &[u8] = program.into();
        let start_at: Address = 0x200_usize.into();
        let end_at: Address = (0x200_usize + input.len()).into();
        self.memory.load(start_at, end_at, input);
    }
    fn load_font(&mut self) {
        let font: [u8; 80] = [
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

        let start_at: Address = 0x50_usize.into();
        let end_at: Address = 0x9F_usize.into();
        self.memory.load(start_at, end_at, font.as_ref());
    }

    fn fetch(&mut self) -> Opcode {
        let opcode: Opcode = (self.memory[self.pc], self.memory[self.pc + 1_usize.into()])
            .try_into()
            .unwrap();
        self.pc += 2_usize.into();
        opcode
    }

    fn execute(&mut self) -> Option<Display> {
        let opcode = self.fetch();
        let mut screen = None::<Display>;
        match opcode {
            Opcode::Cls => screen = Some(Display::new()),
            Opcode::Ret => self.pc = self.stack.pop(),
            Opcode::JpAddress(address) => self.pc = address,
            Opcode::CallAddress(address) => self.stack.push(address),
            Opcode::SeVxByte(x, b) => {
                if self.v[x] == b {
                    self.pc += Address(2);
                }
            }
            Opcode::SneVxByte(x, b) => {
                if self.v[x] != b {
                    self.pc += Address(2);
                }
            }
            Opcode::SeVxVy(x, y) => {
                if self.v[x] == self.v[y] {
                    self.pc += Address(2);
                }
            }
            Opcode::LdVxByte(x, b) => self.v[x] = b,
            Opcode::AddVxByte(x, b) => self.v[x] += b,
            Opcode::LdVxVy(x, y) => self.v[x] = self.v[y],
            Opcode::OrVxVy(x, y) => self.v[x] |= self.v[y],
            Opcode::AndVxVy(x, y) => self.v[x] &= self.v[y],
            Opcode::XorVxVy(x, y) => self.v[x] ^= self.v[y],
            Opcode::AddVxVy(x, y) => {
                let (value, overflow) = self.v[x].overflowing_add(self.v[y]);
                self.v[x] += value;
                self.v[Self::F] = overflow.into();
            }
            Opcode::SubVxVy(x, y) => {
                if self.v[x] > self.v[y] {
                    self.v[Self::F] = 1;
                } else {
                    self.v[Self::F] = 0;
                }
                self.v[x].wrapping_sub(self.v[y]);
            }
            Opcode::ShrVxVy(x, y) => {
                self.v[Self::F] = self.v[x] & 0x1;
                self.v[x] >>= 0x1;
            }
            Opcode::SubnVxVy(x, y) => {
                let (value, underflow) = self.v[x].overflowing_sub(self.v[y]);
                self.v[x] = value;
                self.v[Self::F] = underflow.into()
            }
            Opcode::ShlVxVy(x, y) => {
                self.v[Self::F] = self.v[x] & 0b10000000;
                self.v[x] <<= 0x1;
            }
            Opcode::SneVxVy(x, y) => {
                if self.v[x] != self.v[y] {
                    self.pc += Address(2);
                }
            }
            Opcode::LdIAddress(address) => self.i = address.into(),
            Opcode::JpV0Address(address) => {
                self.pc = address + self.v[Register::<T>::from(0_usize)].into()
            }
            Opcode::RndVxByte(x, b) => {
                self.v[x] = SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .unwrap()
                    .as_nanos() as u8
                    & b
            }
            Opcode::DrwVxVyNibble(x, y, n) => {
                let mut display = Display::new();
                let x = self.v[x];
                let y = self.v[y];
                let n = self.v[Register::<T>::from(n)];

                let mut collision = false;
                for i in 0..n {
                    let i: Address = self.i.into();
                    let sprite = self.memory[self.i.into() + i.into()];
                    for j in 0..8_u8 {
                        let bit = (sprite >> (7 - j)) & 1;
                        let x = (x + j) % 128;
                        let y = (y + i) % 64;
                        if bit == 1 {
                            if display.data[x as usize][y as usize] == 1 {
                                collision = true;
                            }
                            display.data[x as usize][y as usize] ^= 0x1;
                        }
                    }
                }
            }
            Opcode::SkpVx(_) => todo!(),
            Opcode::SknpVx(_) => todo!(),
            Opcode::LdVxDt(_) => todo!(),
            Opcode::LdVxK(_) => todo!(),
            Opcode::LdDtVx(_) => todo!(),
            Opcode::LdStVx(_) => todo!(),
            Opcode::AddIVx(_) => todo!(),
            Opcode::LdFVx(_) => todo!(),
            Opcode::LdBVx(_) => todo!(),
            Opcode::LdIVx(_) => todo!(),
            Opcode::LdVxI(_) => todo!(),
        }
        screen
    }
}

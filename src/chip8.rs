use crate::{opcode::Opcode, screen::Screen, stack::Stack};

pub struct Chip8 {
    memory: [u8; 4096],
    pc: usize,
    i: usize,
    stack: Stack,
    v: [u8; 16],
}

impl Chip8 {
    pub fn new() -> Self {
        let mut chip = Self {
            memory: [0; 4096],
            pc: 0x200, // 512
            i: 0,
            stack: Stack::new(),
            v: [0; 16],
        };
        chip.load_font();
        chip
    }

    pub fn load_program<'a, T: Into<&'a [u8]>>(&mut self, program: T) {
        let input: &[u8] = program.into();
        self.memory[0x200..(0x200 + input.len())].copy_from_slice(input);
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
        self.memory[0x50..=0x9F].copy_from_slice(&font);
    }

    fn fetch(&mut self) -> Opcode {
        let opcode: Opcode = (self.memory[self.pc], self.memory[self.pc + 1])
            .try_into()
            .unwrap();
        self.pc += 2;
        opcode
    }

    fn execute(&mut self, opcode: Opcode) {
        use crate::opcode::{OpcodeData::*, OpcodeName::*};
        let mut screen = None::<Screen>;
        todo!()
    }
}

impl Default for Chip8 {
    fn default() -> Self {
        Self {
            memory: [0; 4096],
            pc: 0x200,
            i: Default::default(),
            stack: Stack::new(),
            v: [0; 16],
        }
    }
}

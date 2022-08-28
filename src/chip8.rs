use crate::stack::Stack;

pub struct Chip8 {
    memory: [u8; 4096],
    pc: usize,
    i: usize,
    stack: Stack,
    v: [u8; 16],
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

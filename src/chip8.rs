use crate::{
    address::Address,
    memory::Memory,
    opcode::Opcode,
    register::{Register, Registers, I, X, Y},
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
    pub fn new() -> Self {
        let mut chip = Self {
            memory: Memory::new(),
            pc: 0x200_usize.into(), // 512
            i: 0.into(),
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
                    self.pc += 2_usize.into();
                }
            }
            Opcode::SneVxByte(x, b) => todo!(),
            Opcode::SeVxVy(_, _) => todo!(),
            Opcode::LdVxByte(_, _) => todo!(),
            Opcode::AddVxByte(_, _) => todo!(),
            Opcode::LdVxVy(_, _) => todo!(),
            Opcode::OrVxVy(_, _) => todo!(),
            Opcode::AndVxVy(_, _) => todo!(),
            Opcode::XorVxVy(_, _) => todo!(),
            Opcode::AddVxVy(_, _) => todo!(),
            Opcode::SubVxVy(_, _) => todo!(),
            Opcode::ShrVxVy(_, _) => todo!(),
            Opcode::SubnVxVy(_, _) => todo!(),
            Opcode::ShlVxVy(_, _) => todo!(),
            Opcode::SneVxVy(_, _) => todo!(),
            Opcode::LdIAddress(_) => todo!(),
            Opcode::JpV0Address(_) => todo!(),
            Opcode::RndVxByte(_, _) => todo!(),
            Opcode::DrwVxVyNibble(_, _, _) => todo!(),
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

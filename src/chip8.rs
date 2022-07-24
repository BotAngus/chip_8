use std::time::{Duration, SystemTime};

#[allow(dead_code)]
pub struct Chip8<const R: usize = 4096, const X: usize = 128, const Y: usize = 64> {
    /// Default 4 kilobytes of RAM
    memory: [u8; R],
    /// Points to current instruction
    pc: usize,

    /// 64 x 32 pixels, monochrome
    display: [[u8; Y]; X],
    /// Only draw if true
    draw: bool,
    /// Index register
    i_reg: u16,
    /// Variable registers
    v_reg: [u8; 16],
    /// 12/16-bit stack
    stack: [u16; 32],

    /// 60Hz
    delay_timer: Duration,
    sound_timer: Duration,
}

#[macro_export]
macro_rules! new {
    ($R:expr, $X:expr, $Y:expr) => {
        chip8::Chip8::<$R, $X, $Y>::new()
    };
}
pub(crate) use new;
use sdl2::{pixels::Color, rect::Point};

use crate::opcode::Opcode;
#[allow(dead_code)]
impl<const R: usize, const X: usize, const Y: usize> Chip8<R, X, Y> {
    pub fn new() -> Chip8<R, X, Y> {
        let mut chip8 = Chip8::<R, X, Y> {
            memory: [0; R],
            pc: 0x200,

            display: [[0; Y]; X],
            draw: false,
            i_reg: 0,
            v_reg: [0; 16],
            stack: [0; 32],
            delay_timer: Duration::from_secs_f32(1.0 / 60.0),
            sound_timer: Duration::from_secs_f32(1.0 / 60.0),
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

    pub fn run(mut self, scale: u32) {
        let (tx, rx) = std::sync::mpsc::channel::<[[u8; Y]; X]>();

        // let delay_time = self.delay_timer;
        // update display
        std::thread::spawn(move || {
            let sdl_context = sdl2::init().unwrap();
            let video_subsystem = sdl_context.video().unwrap();

            let window = video_subsystem
                .window("chip8", X as u32 * scale, Y as u32 * scale)
                .position_centered()
                .borderless()
                .build()
                .unwrap();

            let mut canvas = window.into_canvas().build().unwrap();

            canvas.set_scale(scale as f32, scale as f32).unwrap();
            canvas.clear();
            loop {
                let display = rx.recv().unwrap();
                for x in 0..display.len() {
                    for y in 0..display[x].len() {
                        let colour = display[x][y];
                        canvas.set_draw_color(match colour {
                            0 => Color::WHITE,
                            1 => Color::BLACK,
                            _ => unimplemented!(),
                        });
                        canvas.draw_point(Point::new(x as i32, y as i32)).unwrap();
                    }
                }
                canvas.present();
            }
        });

        loop {
            std::thread::sleep(Duration::from_secs_f32(1.0 / 6.0));

            let opcode = self.fetch();

            self.decode(opcode);

            if self.draw {
                self.draw = false;

                tx.send(self.display).unwrap();
            }
        }
    }

    fn fetch(&mut self) -> Opcode {
        // Bitwise shift the first part of the instruction by 8 and then bitwise OR it with the second part
        let opcode = Opcode::from((self.memory[self.pc], self.memory[self.pc + 1]));

        self.pc += 2;
        opcode
    }

    fn decode(&mut self, opcode: Opcode) {
        // Opcodes from https://en.wikipedia.org/wiki/CHIP-8
        // Bitewise shift the opcode 12 to the right to get the first nibble (half-byte)
        match opcode.first() {
            0x0 => match opcode.as_u16() {
                // 00E0 Clear Screen
                0x00E0 => {
                    self.display = [[0; Y]; X];
                }

                // 00EE
                0x00EE => todo!(),

                _ => {
                    unreachable!()
                }
            },
            // 1NNN Jump to NNN (Set pc to NNN)
            0x1 => self.pc = opcode.nnn() as usize,

            // 2NNN Call subroutine at NNN
            0x2 => todo!(),

            // 3XNN Skip next instruction (pc +=2) if VX == NN
            0x3 => {
                if self.v_reg[opcode.x()] == opcode.nn() {
                    self.pc += 2
                }
            }

            // 4XNN Skip next instruction (pc +=2) if VX != NN
            0x4 => {
                if self.v_reg[opcode.x()] != opcode.nn() {
                    self.pc += 2
                }
            }

            // 5XYN Skips the next instruction if VX equals VY
            0x5 => {
                if self.v_reg[opcode.x()] == self.v_reg[opcode.y()] {
                    self.pc += 2;
                }
            }

            // 6XNN Set register VX (where X is the 2nd nibble) to NN
            0x6 => self.v_reg[opcode.x()] = opcode.nn(),

            // 7XNN Add NN to register VX (where X is the 2nd nibble)
            0x7 => self.v_reg[opcode.x()] += opcode.nn(),

            0x8 => match opcode.last() {
                // 8XY0 Sets VX to the value of VY
                0x0 => self.v_reg[opcode.x()] = self.v_reg[opcode.y()],

                // 8XY1 Sets VX to VX | VY
                0x1 => self.v_reg[opcode.x()] |= self.v_reg[opcode.y()],

                // 8XY2 Sets VX to VX and VY
                0x2 => self.v_reg[opcode.x()] &= self.v_reg[opcode.y()],

                // 8XY3 Sets VX to VX ^= VY
                0x3 => self.v_reg[opcode.x()] ^= self.v_reg[opcode.y()],

                // 8XY4 Adds VY to VX. VF is set to 1 when there's a carry, and to 0 when there is not.
                0x4 => {
                    let (value, overflow) =
                        self.v_reg[opcode.x()].overflowing_add(self.v_reg[opcode.y()]);
                    self.v_reg[opcode.x()] = value;
                    self.v_reg[0x0F] = overflow as u8;
                }

                // 8XY5 VY is subtracted from VX. VF is set to 0 when there's a borrow, and 1 when there is not
                0x5 => {
                    let (value, underflow) =
                        self.v_reg[opcode.x()].overflowing_sub(self.v_reg[opcode.y()]);

                    self.v_reg[opcode.x()] = value;
                    self.v_reg[0x0F] = underflow as u8;
                }

                // 8XY6 Stores the least significant bit of VX in VF and then shifts VX to the right by 1
                0x6 => {
                    self.v_reg[0x0F] = self.v_reg[opcode.x()] & 1; // get least significant bit
                    self.v_reg[opcode.x()] >>= 1;
                }
                // 8XY7 Sets VX to VY minus VX. VF is set to 0 when there's a borrow, and 1 when there is not
                0x7 => {
                    let (value, underflow) =
                        self.v_reg[opcode.y()].overflowing_sub(self.v_reg[opcode.x()]);

                    self.v_reg[opcode.y()] = value;
                    self.v_reg[0x0F] = underflow as u8;
                }

                // 8XY8 Stores the most significant bit of VX in VF and then shifts VX to the left by 1
                0xE => {
                    self.v_reg[0x0F] = (self.v_reg[opcode.y()] >> (u8::BITS - 1)) & 1; // get most significant bit
                    self.v_reg[opcode.y()] <<= 1;
                }
                _ => unreachable!(),
            },
            // 9XY0 Skips the next instruction if VX does not equal VY
            0x9 => {
                if self.v_reg[opcode.x()] != self.v_reg[opcode.y()] {
                    self.pc += 2;
                }
            }

            // ANNN Set index register
            0xA => self.i_reg = opcode.nnn(),

            // Jumps to the address NNN plus V0
            0xB => self.pc = opcode.nnn() as usize + self.v_reg[0x00] as usize,

            // CXNN Sets VX to the result of a bitwise and operation on a random number  and NN
            0xC => {
                self.v_reg[opcode.x()] = opcode.nn()
                    & SystemTime::now()
                        .duration_since(SystemTime::UNIX_EPOCH)
                        .unwrap()
                        .as_micros() as u8
            }
            // DXYN Draw
            0xD => {
                // Set 0x0F of the V register to 0
                // Will be set to 1 if a pixel has been flipped
                self.v_reg[0x0F] = 0;

                // Height of the sprite
                let height: usize = opcode.n();

                // x coordinate to draw at stored in the VX register
                let x = self.v_reg[opcode.x()] as usize;
                // y coordinate to draw at stored in the VY register
                let y = self.v_reg[opcode.y()] as usize;

                // Start putting the values to the buffer
                for byte in 0..height {
                    let pixel = self.memory[self.i_reg as usize + byte];
                    for bit in 0..8_usize {
                        if pixel & (0x80 >> bit) != 0 {
                            if self.display[(x + bit) % X][(y + byte) % Y] > 0 {
                                self.v_reg[0x0F] = 1
                            }
                            self.display[(x + bit) % X][(y + byte) % Y] ^= 1;
                        }
                        // x = (x + bit) % X;
                        // let colour = ;
                        // self.v_reg[0x0F] |= colour & self.display[x][y];

                        // self.display[x][y] ^= colour;
                    }
                }
                self.draw = true;
            }

            0xE => match opcode.last_two() {
                // EX9E Skips the next instruction if the key stored in VX is pressed
                0x9E => {
                    todo!();
                }

                // EXA1 Skips the next instruction if the key stored in VX is not pressed
                0xA1 => todo!(),

                _ => unreachable!(),
            },

            0xF => match opcode.last() {
                // FX07 Sets VX to the value of the delay timer
                0x7 => todo!(),

                _ => unreachable!(),
            },

            _ => unreachable!(),
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

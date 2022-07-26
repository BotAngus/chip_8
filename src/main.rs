#![deny(clippy::all)]

use std::io::Read;

mod chip8;
mod input;
mod opcode;
mod stack;

fn main() -> Result<(), Box<dyn std::error::Error>> {

        
    
    // let args: Vec<String> = std::env::args().collect();

    // let mut file = std::fs::File::open(&args[1])?;

    // let mut program = Vec::new();

    // file.read_to_end(&mut program)?;

    // let mut chip8 = chip8::new!(4096, 128, 64);
    // chip8.load_program(program.as_ref());
    // chip8.run()?;

    Ok(())
}


// for byte in 0..height {
//     let pixel = self.memory[self.i_reg as usize + byte];
//     for bit in 0..8_usize {
//         if pixel & (0x80 >> bit) != 0 {
//             if self.display[(x + bit) % X][(y + byte) % Y] > 0 {
//                 self.v_reg[0x0F] = 1
//             }
//             self.display[(x + bit) % X][(y + byte) % Y] ^= 1;
//         }
//     }
// }

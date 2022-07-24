#![deny(clippy::all)]

use std::io::Read;

mod chip8;
mod opcode;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = std::fs::File::open("./roms/IBM_Logo.ch8")?;

    let mut program = Vec::new();

    file.read_to_end(&mut program)?;
    let mut chip8 = chip8::new!(4096, 128, 64);
    chip8.load_program(program.as_ref());
    chip8.run(8);

    Ok(())
}

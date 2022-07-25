#![deny(clippy::all)]

use std::io::Read;

mod chip8;
mod keyboard;
mod opcode;
mod stack;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();

    let mut file = std::fs::File::open(&args[1])?;

    let mut program = Vec::new();

    file.read_to_end(&mut program)?;
    let mut chip8 = chip8::new!(4096, 128, 64);
    chip8.load_program(program.as_ref());
    chip8.run(8)?;

    Ok(())
}

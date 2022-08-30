#![deny(clippy::all)]
#![deny(clippy::pedantic)]

use chip8::Chip8;

mod opcode;
mod chip8;
mod stack;
mod screen;
mod nibble;
mod address;
mod memory;
mod register;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _chip = Chip8::new();
    

    Ok(())
}

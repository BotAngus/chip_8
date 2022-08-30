#![deny(clippy::all)]
#![deny(clippy::pedantic)]

mod opcode;
mod chip8;
mod stack;
mod screen;
mod nibble;
mod address;
mod memory;
mod register;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

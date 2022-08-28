#![deny(clippy::all)]
#![deny(clippy::pedantic)]

mod opcode;
mod chip8;
mod stack;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

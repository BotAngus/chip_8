#![deny(clippy::all)]

use std::io::Read;

use chip8::font;
use sdl2::{pixels::Color, rect::Point};

mod chip8;
mod input;
mod opcode;
mod stack;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();

    let mut file = std::fs::File::open(&args[1])?;

    let mut program = Vec::new();

    file.read_to_end(&mut program)?;

    let mut chip8 = chip8::new!(4096, 128, 64);
    chip8.load_font(&font::STANDARD);
    chip8.load_program(program.as_ref());

    let sdl_context = sdl2::init()?;

    // let mut event_pump = sdl_context.event_pump()?;

    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem
        .window(
            "Chip 8",
            chip8.width() as u32 * 4,
            chip8.height() as u32 * 4,
        )
        .borderless()
        .position_centered()
        .build()?;
    let mut canvas = window.into_canvas().build()?;
    canvas.set_scale(4 as f32, 4 as f32)?;
    canvas.clear();
    loop {
        std::thread::sleep(std::time::Duration::from_secs_f32(1.0 / 60.0));
        let state = chip8.tick();

        if let Some(display) = state.display {
            // println!("{:?}", display);
            for (x, _) in display.iter().enumerate() {
                for y in 0..display[x].len() {
                    let colour = display[x][y];
                    canvas.set_draw_color(match colour {
                        0 => Color::WHITE,
                        1 => Color::BLACK,
                        _ => unimplemented!(),
                    });
                    canvas.draw_point(Point::new(x as i32, y as i32))?;
                }
            }
            canvas.present();
        }
    }
    // Ok(())
}

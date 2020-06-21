use chip8::Chip8;
use std::error::Error;
use std::{time, thread};

mod chip8;
mod drivers;

use drivers::DisplayDriver;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

fn main() -> Result<(), Box<dyn Error>> {
    let sdl_context = sdl2::init()?;
    let display_driver = DisplayDriver::new(&sdl_context)?;
    let mut chip8 = Chip8::new(display_driver);
    let rom_path = PathBuf::from("roms/Space Invaders [David Winter].ch8");
    chip8.load(rom_path);

    let mut event_pump = sdl_context.event_pump()?;
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                }
                Event::KeyDown { keycode: Some(Keycode::Num1), .. } => {
                    chip8.set_keys(0x1, true);
                }
                _ => {}
            }
        }
        chip8.cycle();

        if chip8.draw_flag {
            chip8.draw()
        }

        // chip8.set_keys();

        // Execute 60 opcodes in one second
        let ten_millis = time::Duration::from_millis(1000 / 60);

        thread::sleep(ten_millis)
    }
    Ok(())
}

use chip8::Chip8;
use std::error::Error;
use std::{time, thread};

mod chip8;
mod drivers;

use drivers::DisplayDriver;
use std::path::PathBuf;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

fn main() -> Result<(), Box<dyn Error>> {
    let sdl_context = sdl2::init()?;
    let display_driver = DisplayDriver::new(&sdl_context)?;
    let mut chip8 = Chip8::new(display_driver);
    let rom_path = PathBuf::from("roms/Space Invaders [David Winter].ch8");
    chip8.load(rom_path);
    let sleep_duration = time::Duration::from_millis(2);
    let mut event_pump = sdl_context.event_pump()?;
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                }
                Event::KeyDown { keycode: Some(Keycode::Num1), .. } => {
                    chip8.set_keys(0x5, true);
                }
                Event::KeyUp { keycode: Some(Keycode::Num1), .. } => {
                    chip8.set_keys(0x5, false);
                }
                _ => {}
            }
        }
        chip8.cycle();

        if chip8.draw_flag {
            chip8.draw()
        }

        thread::sleep(sleep_duration)
    }
    Ok(())
}

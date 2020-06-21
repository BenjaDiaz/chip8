use chip8::Chip8;
use std::error::Error;
use std::{time, thread};

mod chip8;
mod drivers;

use drivers::DisplayDriver;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

fn main() -> Result<(), Box<dyn Error>> {
    let display_driver = DisplayDriver::new()?;
    let mut chip8 = Chip8::new(display_driver);
    let rom_path = PathBuf::from("roms/IBM Logo.ch8");
    chip8.load(rom_path);

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    }).expect("Error setting Ctrl-C handler");

    println!("Waiting for Ctrl-C...");
    while running.load(Ordering::SeqCst) {
        chip8.cycle();

        if chip8.draw_flag {
            chip8.draw()
        }

        chip8.set_keys();

        // Execute 60 opcodes in one second
        let ten_millis = time::Duration::from_millis(1000 / 60);

        thread::sleep(ten_millis);
    }
    println!("Got it! Exiting...");
    Ok(())
}

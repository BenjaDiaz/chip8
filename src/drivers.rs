use sdl2::render::WindowCanvas;
use std::error::Error;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::Sdl;

const CHIP8_WIDTH: u32 = 64;
const CHIP8_HEIGHT: u32 = 32;
const CHIP8_SCALE: u32 = 8;

pub struct DisplayDriver {
    canvas: WindowCanvas
}

impl DisplayDriver {
    pub fn new(sdl_context: &Sdl) -> Result<DisplayDriver, Box<dyn Error>> {
        let video_subsystem = sdl_context.video()?;

        let window = video_subsystem.window("chip8", CHIP8_WIDTH * CHIP8_SCALE, CHIP8_HEIGHT * CHIP8_SCALE)
            .position_centered()
            .build()
            .expect("could not initialize video subsystem");

        let mut canvas = window.into_canvas().build()
            .expect("could not make a canvas");
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.present();
        Ok(DisplayDriver {
            canvas
        })
    }

    pub fn draw(&mut self, gfx: [u8; 64 * 32]) {
        self.canvas.clear();
        let mut x: i32;
        let mut y: i32;
        let mut r;
        for (i, elem) in gfx.iter().enumerate() {
            if *elem == 1 {
                self.canvas.set_draw_color(Color::RGB(255, 255, 255));
            } else {
                self.canvas.set_draw_color(Color::RGB(0, 0, 0));
            }
            x = ((i % CHIP8_WIDTH as usize) * CHIP8_SCALE as usize) as i32;
            y = ((i / CHIP8_WIDTH as usize) * CHIP8_SCALE as usize) as i32;

            r = Rect::new(x, y, CHIP8_SCALE, CHIP8_SCALE);
            self.canvas.fill_rect(r).expect("could not draw rect");
        }
        self.canvas.present();
    }
}

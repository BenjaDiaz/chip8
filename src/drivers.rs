use sdl2::render::WindowCanvas;
use std::error::Error;
use sdl2::pixels::Color;
use sdl2::rect::Point;
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

        canvas.clear();
        canvas.present();
        Ok(DisplayDriver {
            canvas
        })
    }

    pub fn draw(&mut self, gfx: [u8; 64 * 32]) {
        // TODO: Traverse gfx and draw pixels
        self.canvas.set_draw_color(Color::RGB(255, 255, 255));
        let mut x:i32;
        let mut y:i32;
        let mut p;
        for (i, elem) in gfx.iter().enumerate() {
            if *elem == 1 {
                x = ((i % CHIP8_WIDTH as usize) * CHIP8_SCALE as usize) as i32;
                y = ((i / CHIP8_WIDTH as usize) * CHIP8_SCALE as usize) as i32;
                for j in 0..CHIP8_SCALE {
                    for k in 0..CHIP8_SCALE {
                        p = Point::new((x + j as i32), (y + k as i32));
                        self.canvas.draw_point(p).expect("could not draw point");
                    }
                }
            }
        }
        self.canvas.present();
    }

    pub fn clear(&mut self) {
        self.canvas.clear();
    }
}

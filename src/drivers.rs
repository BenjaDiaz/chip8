use sdl2::render::WindowCanvas;
use std::error::Error;
use sdl2::pixels::Color;
use sdl2::rect::Point;

pub struct DisplayDriver {
    canvas: WindowCanvas
}

impl DisplayDriver {
    pub fn new() -> Result<DisplayDriver, Box<dyn Error>> {
        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;

        let window = video_subsystem.window("chip8", 64, 32)
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
        let mut x;
        let mut y;
        let mut p;
        for (i, elem) in gfx.iter().enumerate() {
            if *elem == 1 {
                x = i % 64;
                y = i / 64;
                p = Point::new(x as i32, y as i32);
                self.canvas.draw_point(p).expect("could not draw point");
            }
        }
        self.canvas.present();
    }

    pub fn clear(&mut self) {
        self.canvas.clear();
    }
}

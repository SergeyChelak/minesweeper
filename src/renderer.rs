use sdl2::{render::WindowCanvas, video::Window};
use sdl2::pixels::Color;
use std::time::Duration;

pub struct Renderer {
    canvas: WindowCanvas,
}

impl Renderer {
    pub fn with_window(window: Window) -> Result<Self, String> {
        let canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
        Ok(Self { canvas })
    }

    pub fn draw(&mut self) -> Result<(), String> {
        self.canvas.set_draw_color(Color::BLACK);
        self.canvas.clear();
        // TODO: draw code
        self.canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
        Ok(())
    }
}

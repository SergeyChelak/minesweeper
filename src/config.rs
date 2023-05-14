use sdl2::pixels::Color;

pub struct Configuration;

impl Configuration {
    pub fn window_title(&self) -> String {
        "Minesweeper".to_string()
    }

    pub fn window_height(&self) -> u32 {
        640
    }

    pub fn window_width(&self) -> u32 {
        1024
    }

    pub fn target_fps(&self) -> u64 {
        10
    }

    pub fn color_background(&self) -> Color {
        Color::RGB(0, 0, 0)
    }
}

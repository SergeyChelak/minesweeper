use sdl2::pixels::Color;

pub struct ColorManager;

impl ColorManager {
    pub fn background(&self) -> Color {
        Color::RGB(0, 0, 0)
    }
}

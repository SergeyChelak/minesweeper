use sdl2::pixels::Color;

pub struct ColorManager;

impl ColorManager {
    pub fn background(&self) -> Color {
        Color::RGB(0, 0, 0)
    }

    pub fn title_message(&self) -> Color {
        Color::RGB(50, 50, 190)
    }
}

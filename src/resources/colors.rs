use sdl2::pixels::Color;

pub struct ColorManager;

impl ColorManager {
    pub fn title_message(&self) -> Color {
        Color::RGB(150, 150, 90)
    }

    pub fn fade(&self) -> Color {
        Color::RGBA(0, 0, 0, 170)
    }
}

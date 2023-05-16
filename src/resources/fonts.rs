use std::rc::Rc;

use sdl2::ttf::Font;

use super::{FontDetails, FontManager};

type FontLoadResult<'a> = Result<Rc<Font<'a, 'static>>, String>;

pub trait FontProvider<'a> {
    fn font_header(&mut self) -> FontLoadResult<'a>;
}

impl<'a> FontProvider<'a> for FontManager<'a> {
    fn font_header(&mut self) -> FontLoadResult<'a> {
        self.load(&FontDetails {
            path: "assets/fonts/Freedom-10eM.ttf".to_string(),
            size: 170,
        })
    }
}

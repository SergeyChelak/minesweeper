use sdl2::render::Texture;
use std::rc::Rc;

use super::TextureManager;

type TextureLoadResult<'a> = Result<Rc<Texture<'a>>, String>;

pub trait TextureProvider<'a> {
    fn img_empty(&mut self) -> TextureLoadResult<'a>;
    fn img_bomb(&mut self) -> TextureLoadResult<'a>;
    fn img_flag(&mut self) -> TextureLoadResult<'a>;
    fn img_unknown(&mut self) -> TextureLoadResult<'a>;
    fn img_number(&mut self, num: usize) -> TextureLoadResult<'a>;
}

impl<'a, T> TextureProvider<'a> for TextureManager<'a, T> {
    fn img_empty(&mut self) -> TextureLoadResult<'a> {
        self.load("assets/images/empty_64x64.png")
    }

    fn img_bomb(&mut self) -> TextureLoadResult<'a> {
        self.load("assets/images/bomb_64x64.png")
    }

    fn img_flag(&mut self) -> TextureLoadResult<'a> {
        self.load("assets/images/flag_1_64x64.png")
    }

    fn img_unknown(&mut self) -> TextureLoadResult<'a> {
        self.load("assets/images/unknown_1_64x64.png")
    }

    fn img_number(&mut self, num: usize) -> TextureLoadResult<'a> {
        let path = format!("assets/images/{num}_64x64.png");
        self.load(&path)
    }
}
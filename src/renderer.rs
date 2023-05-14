use std::collections::HashMap;
use std::rc::Rc;

use sdl2::pixels::Color;
use sdl2::render::Texture;
use sdl2::{render::WindowCanvas, video::Window};

use crate::game_context::*;

pub struct Renderer {}

impl Renderer {
    pub fn new() -> Result<Self, String> {
        // let image_resources = Self::load_images()?;

        Ok(Self {})
    }

    pub fn draw(&mut self, game_context: &GameContext) -> Result<(), String> {
        match game_context.state() {
            State::InProgress => self.draw_board(game_context),
            State::Lose => self.draw_lose(game_context),
            State::Win => self.draw_win(game_context),
        }
    }

    fn draw_board(&mut self, game_context: &GameContext) -> Result<(), String> {
        let (rows, cols) = game_context.board_size();
        for col in 0..cols {
            for row in 0..rows {
                let cell = game_context.get_cell(row, col);
                let key = if !cell.is_visible() {
                    "hidden".to_string()
                } else if cell.is_flagged() {
                    "flagged".to_string()
                } else if cell.is_safe() {
                    match cell.mines_count() {
                        0 => "blank".to_string(),
                        count => format!("image_{count}"),
                    }
                } else {
                    "bomb".to_string()
                };
                // TODO: get image for key
            }
        }
        Ok(())
    }

    fn draw_win(&mut self, game_context: &GameContext) -> Result<(), String> {
        self.draw_board(game_context)?;
        Ok(())
    }

    fn draw_lose(&mut self, game_context: &GameContext) -> Result<(), String> {
        self.draw_board(game_context)?;
        Ok(())
    }
}

use std::time::{Duration, Instant};

use sdl2::EventPump;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::{Rect, Point};
use sdl2::video::WindowContext;
use sdl2::{render::WindowCanvas};

use crate::resources::{ColorManager, TextureManager, FontManager, TextureProvider};
use crate::{game_model::*};

pub struct Minesweeper<'a> {
    model: GameModel,
    canvas: WindowCanvas,
    texture_manager: TextureManager<'a, WindowContext>,
    font_manager: FontManager<'a>,
    color_manager: ColorManager,
    event_pump: EventPump,
    target_fps: u64,
    is_running: bool,
}

impl<'a> Minesweeper<'a> {
    pub fn new(
        model: GameModel, 
        canvas: WindowCanvas,
        texture_manager: TextureManager<'a, WindowContext>,
        font_manager: FontManager<'a>,
        color_manager: ColorManager,
        event_pump: EventPump
    ) -> Self {
        Self {
            model,
            canvas,
            texture_manager,
            font_manager,
            color_manager,
            event_pump,
            target_fps: 5,
            is_running: false,
        }
    }

    pub fn run(&mut self) -> Result<(), String> {
        let target_frame_duration = Duration::from_millis(1000u64 / self.target_fps);
        self.is_running = true;
        loop {
            let frame_start_time = Instant::now();
            self.handle_events();
            if !self.is_running {
                break;
            }
            self.draw()?;
            let elapsed_time = frame_start_time.elapsed();
            let sleep_time = target_frame_duration.saturating_sub(elapsed_time);
            if sleep_time.is_zero() {
                println!("Frame elapsed time {elapsed_time:?}");
                continue;
            }
            ::std::thread::sleep(sleep_time);
        }
        Ok(())
    }

    pub fn handle_events(&mut self) {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => self.is_running = false,
                _ => {}
            }
        }
    }
    
    pub fn draw(&mut self) -> Result<(), String> {
        self.canvas.set_draw_color(self.color_manager.background());
        self.canvas.clear();
        match self.model.state() {
            State::InProgress => self.draw_board(),
            State::Lose => self.draw_lose(),
            State::Win => self.draw_win(),
        }?;
        self.canvas.present();
        Ok(())
    }

    fn draw_board(&mut self) -> Result<(), String> {
        let image_height = 64u32;
        let image_width = 64u32;

        let (rows, cols) = self.model.board_size();
        for col in 0..cols {
            for row in 0..rows {
                let cell = self.model.get_cell(row, col);
                let texture = if !cell.is_visible() {
                    self.texture_manager.img_unknown()
                } else if cell.is_flagged() {
                    self.texture_manager.img_flag()
                } else if cell.is_safe() {
                    match cell.mines_count() {
                        0 => self.texture_manager.img_empty(),
                        count => self.texture_manager.img_number(count),
                    }
                } else {
                    self.texture_manager.img_bomb()
                }?;
                let src = Rect::new(0, 0, image_width, image_height);
                let w = image_width as i32;
                let h = image_height as i32;
                let center = Point::new(w / 2, h / 2);
                let dest = Rect::new(col as i32 * w, row as i32 * w, image_width, image_height);
                self.canvas.copy_ex(&texture, src, dest, 0.0, center, false, false)?;
            }
        }
        Ok(())
    }

    fn draw_win(&mut self) -> Result<(), String> {
        self.draw_board()?;
        Ok(())
    }

    fn draw_lose(&mut self) -> Result<(), String> {
        self.draw_board()?;
        Ok(())
    }
}

use std::collections::HashSet;
use std::time::{Duration, Instant};

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::rect::{Point, Rect};
use sdl2::render::TextureCreator;
use sdl2::render::WindowCanvas;
use sdl2::video::WindowContext;
use sdl2::EventPump;

use crate::game_model::*;
use crate::resources::{ColorManager, FontManager, FontProvider, TextureManager, TextureProvider};

pub struct Size {
    pub height: u32,
    pub width: u32,
}

pub struct Minesweeper<'a> {
    model: GameModel,
    canvas: WindowCanvas,
    texture_creator: TextureCreator<WindowContext>,
    texture_manager: TextureManager<'a, WindowContext>,
    font_manager: FontManager<'a>,
    color_manager: ColorManager,
    event_pump: EventPump,
    target_fps: u64,
    is_running: bool,
    cell_size: Size,
    window_size: Size,
    prev_mouse_buttons: HashSet<MouseButton>,
}

impl<'a> Minesweeper<'a> {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        model: GameModel,
        canvas: WindowCanvas,
        texture_creator: TextureCreator<WindowContext>,
        texture_manager: TextureManager<'a, WindowContext>,
        font_manager: FontManager<'a>,
        color_manager: ColorManager,
        event_pump: EventPump,
        cell_size: Size,
        window_size: Size,
    ) -> Self {
        Self {
            model,
            canvas,
            texture_creator,
            texture_manager,
            font_manager,
            color_manager,
            event_pump,
            target_fps: 24,
            is_running: false,
            cell_size,
            window_size,
            prev_mouse_buttons: HashSet::new(),
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
                Event::KeyDown {
                    keycode: Some(Keycode::Space),
                    ..
                } => self.model.restart(),
                _ => {}
            }
        }

        let state = self.event_pump.mouse_state();
        let buttons = state
            .pressed_mouse_buttons()
            .collect::<HashSet<MouseButton>>();
        if buttons.is_empty() && self.prev_mouse_buttons.len() == 1 {
            let (x, y) = (state.x(), state.y());
            if x >= 0 && y >= 0 {
                let (x, y) = (x as u32, y as u32);
                let row = (y / self.cell_size.width) as usize;
                let col = (x / self.cell_size.height) as usize;
                if self.prev_mouse_buttons.contains(&MouseButton::Left) {
                    self.model.open_cell(row, col);
                } else if self.prev_mouse_buttons.contains(&MouseButton::Right) {
                    self.model.flag_cell(row, col);
                }
            }
        }
        self.prev_mouse_buttons = buttons;
    }

    pub fn draw(&mut self) -> Result<(), String> {
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
        let is_lose = self.model.state() == State::Lose;
        let (rows, cols) = self.model.board_size();
        for col in 0..cols {
            for row in 0..rows {
                let cell = self.model.get_cell(row, col);
                let texture = if !cell.is_safe() && is_lose {
                    if self.model.is_last_step(row, col) {
                        self.texture_manager.img_bomb_exploded()
                    } else {
                        self.texture_manager.img_bomb()
                    }
                } else if cell.is_flagged() {
                    self.texture_manager.img_flag()
                } else if cell.is_visible() {
                    match cell.mines_count() {
                        0 => self.texture_manager.img_empty(),
                        count => self.texture_manager.img_number(count),
                    }
                } else {
                    self.texture_manager.img_unknown()
                }?;
                let cell_width = self.cell_size.width;
                let cell_height = self.cell_size.height;
                let src = Rect::new(0, 0, cell_width, cell_height);
                let w = cell_width as i32;
                let h = cell_height as i32;
                let center = Point::new(w / 2, h / 2);
                let dest = Rect::new(col as i32 * w, row as i32 * w, cell_width, cell_height);
                self.canvas
                    .copy_ex(&texture, src, dest, 0.0, center, false, false)?;
            }
        }
        Ok(())
    }

    fn draw_win(&mut self) -> Result<(), String> {
        self.draw_board()?;
        self.show_message("You win")
    }

    fn draw_lose(&mut self) -> Result<(), String> {
        self.draw_board()?;
        self.show_message("Game Over")
    }

    fn show_message(&mut self, text: &str) -> Result<(), String> {
        let (w, h) = (self.window_size.width, self.window_size.height);

        self.canvas.set_blend_mode(sdl2::render::BlendMode::Blend);
        self.canvas.set_draw_color(self.color_manager.fade());
        self.canvas.fill_rect(Rect::new(0, 0, w, h))?;

        self.canvas.set_blend_mode(sdl2::render::BlendMode::None);

        let font = self.font_manager.font_header()?;
        let surface = font
            .render(text)
            .blended(self.color_manager.title_message())
            .map_err(|e| e.to_string())?;
        let texture = self
            .texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())?;

        let size = Size {
            height: 150,
            width: 650,
        };
        let x = (w - size.width) / 2;
        let y = (h - size.height) / 2;
        let frame = Rect::new(x as i32, y as i32, size.width, size.height);
        self.canvas.copy(&texture, None, Some(frame))
    }
}

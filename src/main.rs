extern crate sdl2;

mod game_model;
use game_model::GameModel;

mod resources;
use resources::*;

mod minesweeper;
use minesweeper::Minesweeper;

mod config;
use config::Configuration;

fn main() -> Result<(), String> {
    // TODO: config should be loaded
    let config = Configuration;
    let mut game_model = GameModel::new();
    game_model.start(config.row_count(), config.col_count(), config.mines_count());

    // sdl setup
    let window_height = 64 * config.row_count() as u32;
    let window_width = 64 * config.col_count() as u32;
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem
        .window(&config.window_title(), window_width, window_height)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;
    let canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    let ttf_context = sdl2::ttf::init().map_err(|err| err.to_string())?;
    let font_manager = resources::FontManager::new(&ttf_context);

    let texture_creator = canvas.texture_creator();
    let texture_manager = resources::TextureManager::new(&texture_creator);
    
    let event_pump = sdl_context.event_pump()?;

    let color_manager = ColorManager;

    let texture_creator = canvas.texture_creator();

    let mut minesweeper = Minesweeper::new(
        game_model,
        canvas,
        texture_creator,
        texture_manager,
        font_manager,
        color_manager,
        event_pump,
    );
    minesweeper.run()
}
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

    let mut minesweeper = Minesweeper::new(
        game_model, 
        canvas, 
        texture_manager,
        font_manager,
        color_manager,
        event_pump
    );
    minesweeper.run()
}

/*
fn game_loop(config: &Configuration, context: &mut SDLContext) -> Result<(), String> {
    let canvas = &mut context.canvas;
    let target_frame_duration = Duration::from_millis(1000u64 / config.target_fps());

    let mut font_manager = resources::FontManager::new(&context.ttf_context);
    let font = font_manager.font_header()?;
    let mut texture_manager = resources::TextureManager::new(&context.texture_creator);
    let image_1 = texture_manager.img_bomb()?;
    'running: loop {
        let frame_start_time = Instant::now();
        for event in context.event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        {
            canvas.set_draw_color(Color::RGB(0, 0, 0));
            canvas.clear();
            // TODO: draw code

            // draw image
            let h = 64u32;
            let w = 64u32;
            for i in 0..config.col_count() as u32 {
                for j in 0..config.row_count() as u32 {
                    let src = Rect::new(0, 0, w, h);
                    let center = Point::new(w as i32 / 2, h as i32 / 2);
                    let dest = Rect::new((i * w) as i32, (j * w) as i32, w, h);
                    canvas.copy_ex(&image_1, src, dest, 0.0, center, false, false)?;
                }
            }

            // draw text
            let text = "In development".to_string();
            let surface = font
                .render(&text)
                .blended(Color::RGB(240, 0, 30))
                .map_err(|e| e.to_string())?;
            let texture = context
                .texture_creator
                .create_texture_from_surface(&surface)
                .map_err(|e| e.to_string())?;

            let texture_target = Rect::new(130, 20, 650, 150);
            canvas.copy(&texture, None, Some(texture_target))?;

            canvas.present();
        }
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
 */

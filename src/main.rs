extern crate sdl2;

mod game_context;
mod renderer;

use renderer::Renderer;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::video::Window;
use sdl2::Sdl;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let window = setup_window(&sdl_context)?;
    let mut renderer = Renderer::with_window(window)?;

    let mut event_pump = sdl_context.event_pump()?;
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        renderer.draw()?;
    }
    Ok(())
}

fn setup_window(sdl_context: &Sdl) -> Result<Window, String> {
    sdl_context
        .video()?
        .window("Minesweeper", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())
}

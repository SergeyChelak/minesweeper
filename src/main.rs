extern crate sdl2;

mod game_context;

mod resources;
use resources::*;

mod renderer;
use renderer::Renderer;

mod config;
use config::Configuration;

use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{TextureCreator, WindowCanvas};

use sdl2::event::Event;
use sdl2::ttf::Sdl2TtfContext;
use sdl2::video::WindowContext;
use sdl2::EventPump;

use std::time::{Duration, Instant};

struct SDLContext {
    // sdl_context: Sdl,
    canvas: WindowCanvas,
    event_pump: EventPump,
    texture_creator: TextureCreator<WindowContext>,
    ttf_context: Sdl2TtfContext,
}

impl SDLContext {
    fn with_configuration(config: &Configuration) -> Result<Self, String> {
        let context = sdl2::init()?;
        let window = {
            let video_subsystem = context.video()?;
            video_subsystem
                .window(
                    &config.window_title(),
                    config.window_width(),
                    config.window_height(),
                )
                .position_centered()
                .build()
                .map_err(|e| e.to_string())
        }?;
        let event_pump = context.event_pump()?;
        let canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
        let texture_creator = canvas.texture_creator();
        let ttf_context: sdl2::ttf::Sdl2TtfContext =
            sdl2::ttf::init().map_err(|err| err.to_string())?;
        Ok(Self {
            // context,
            canvas,
            event_pump,
            texture_creator,
            ttf_context,
        })
    }
}

fn main() -> Result<(), String> {
    let config = Configuration;
    let mut context = SDLContext::with_configuration(&config)?;
    game_loop(&config, &mut context)
    // Ok(())
}

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
            canvas.set_draw_color(config.color_background());
            canvas.clear();
            // TODO: draw code

            // draw image
            let h = 64u32;
            let w = 64u32;
            for i in 0..16 {
                for j in 0..10 {
                    let src = Rect::new(0, 0, w, h);
                    let center = Point::new(w as i32 / 2, h as i32 / 2);
                    let dest = Rect::new(i * w as i32, j * w as i32, w, h);
                    canvas.copy_ex(&image_1, src, dest, 0.0, center, false, false)?;
                }
            }

            // draw text
            let text = "In development".to_string();
            let surface = font
                .render(&text)
                .blended(Color::RGB(240, 0, 30))
                .map_err(|e| e.to_string())?;
            let texture = context.texture_creator
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

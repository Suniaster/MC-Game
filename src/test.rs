use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use sdl2::rect;

use std::time::Duration;
use std::path::Path;
use sdl2::image::{InitFlag, LoadTexture};

mod test;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    
    // let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG)?;

    let mut position = 10;
    let mut r = rect::Rect::new(10, 10, 100, 100);

    let window = video_subsystem.window("rust-sdl2 demo", 800, 600)
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");

    let mut canvas = window.into_canvas().build()
        .expect("could not make a canvas");

    let texture_creator = canvas.texture_creator();
    let texture = texture_creator.load_texture("./assets/buizel.png")?;


    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump()?;
    let mut i = 0;
    'running: loop {
        i = (i + 1) % 255;
        canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                },
                Event::KeyDown { keycode: Some(Keycode::A), ..} =>{
                    test::teste();
                },
                Event::MouseMotion{x, ..} =>{
                    position = x;
                },
                _ => {}
            }
        }

        r.set_x(position);
        canvas.copy(&texture, None, r)?;
        // The rest of the game loop goes here...
        // canvas.set_draw_color(Color::RGB(0, 0, 0));
        // canvas.fill_rect(r).expect("error");

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
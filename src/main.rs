extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Instant;

mod components;
mod entities;
mod scene;
mod systems;
use view;

// Proximos Objetivos
// - Adicionar Colisao
// - Adicionar Input
// - Ajustar estrutura do loop principal
// - Adicionar fixed dt
// - Decidir como estruturar melhor sistemas em arquivos separados
// - Fazer alguma parada massa

fn main() {
    view::main();
    
    let mut world = scene::GameScene::new((800., 600.));

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    // let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG)?;

    let window = video_subsystem
        .window("rust-sdl2 demo", 800, 600)
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");

    let mut canvas = window
        .into_canvas()
        .build()
        .expect("could not make a canvas");

    let texture_creator: sdl2::render::TextureCreator<sdl2::video::WindowContext> =
        canvas.texture_creator();

    let mut event_pump = sdl_context.event_pump().expect("");
    let mut start = Instant::now();

    world.setup_components();
    world.setup_assets(&texture_creator);

    'running: loop {
        // handle keyboard events

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'running;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::E),
                    ..
                } => {
                    entities::Blob::create(&mut world);
                }
                _ => {}
            }
        }
        // The rest of the game loop goes here...
        let delta_t = start.elapsed().as_micros() as f64 / 1_000_000.;
        start = Instant::now();

        systems::physics_system(&mut world.components, delta_t, world.time_scale);
        systems::circular_world_system(&mut world.components, &world.scene_size);

        systems::render_system(&world.components, &world.assets, &mut canvas);
        print!(
            "\rFPS: {:.3} \t||| Entities: {:?}",
            1. / delta_t,
            world.blobs.len()
        );
    }
    println!();
}

// fn main() -> Result<(), String> {
//     let sdl_context = sdl2::init()?;
//     let video_subsystem = sdl_context.video()?;

//     let window = video_subsystem
//         .window("SDL2", 640, 480)
//         .position_centered()
//         .build()
//         .map_err(|e| e.to_string())?;

//     let mut canvas = window
//         .into_canvas()
//         .accelerated()
//         .build()
//         .map_err(|e| e.to_string())?;
//     let texture_creator = canvas.texture_creator();

//     canvas.set_draw_color(sdl2::pixels::Color::RGBA(0, 0, 0, 255));

//     let timer = sdl_context.timer()?;

//     let mut event_pump = sdl_context.event_pump()?;

//     // animation sheet and extras are available from
//     // https://opengameart.org/content/a-platformer-in-the-forest
//     let temp_surface = sdl2::surface::Surface::load_bmp(Path::new("characters.bmp"))?;
//     let texture = texture_creator
//         .create_texture_from_surface(&temp_surface)
//         .map_err(|e| e.to_string())?;

//     let frames_per_anim = 4;
//     let sprite_tile_size = (32, 32);

//     // Baby - walk animation
//     let mut source_rect_0 = Rect::new(0, 0, sprite_tile_size.0, sprite_tile_size.0);
//     let mut dest_rect_0 = Rect::new(0, 0, sprite_tile_size.0 * 4, sprite_tile_size.0 * 4);
//     dest_rect_0.center_on(Point::new(-64, 120));

//     // King - walk animation
//     let mut source_rect_1 = Rect::new(0, 32, sprite_tile_size.0, sprite_tile_size.0);
//     let mut dest_rect_1 = Rect::new(0, 32, sprite_tile_size.0 * 4, sprite_tile_size.0 * 4);
//     dest_rect_1.center_on(Point::new(0, 240));

//     // Soldier - walk animation
//     let mut source_rect_2 = Rect::new(0, 64, sprite_tile_size.0, sprite_tile_size.0);
//     let mut dest_rect_2 = Rect::new(0, 64, sprite_tile_size.0 * 4, sprite_tile_size.0 * 4);
//     dest_rect_2.center_on(Point::new(440, 360));

//     let mut running = true;
//     while running {
//         for event in event_pump.poll_iter() {
//             match event {
//                 Event::Quit { .. }
//                 | Event::KeyDown {
//                     keycode: Some(Keycode::Escape),
//                     ..
//                 } => {
//                     running = false;
//                 }
//                 _ => {}
//             }
//         }

//         let ticks = timer.ticks() as i32;

//         // set the current frame for time
//         source_rect_0.set_x(32 * ((ticks / 100) % frames_per_anim));
//         dest_rect_0.set_x(1 * ((ticks / 14) % 768) - 128);

//         source_rect_1.set_x(32 * ((ticks / 100) % frames_per_anim));
//         dest_rect_1.set_x((1 * ((ticks / 12) % 768) - 672) * -1);

//         source_rect_2.set_x(32 * ((ticks / 100) % frames_per_anim));
//         dest_rect_2.set_x(1 * ((ticks / 10) % 768) - 128);

//         canvas.clear();
//         // copy the frame to the canvas
//         canvas.copy_ex(
//             &texture,
//             Some(source_rect_0),
//             Some(dest_rect_0),
//             0.0,
//             None,
//             false,
//             false,
//         )?;
//         canvas.copy_ex(
//             &texture,
//             Some(source_rect_1),
//             Some(dest_rect_1),
//             0.0,
//             None,
//             true,
//             false,
//         )?;
//         canvas.copy_ex(
//             &texture,
//             Some(source_rect_2),
//             Some(dest_rect_2),
//             0.0,
//             None,
//             false,
//             false,
//         )?;
//         canvas.present();

//         std::thread::sleep(Duration::from_millis(100));
//     }

//     Ok(())
// }

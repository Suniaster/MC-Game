use super::components::*;
use super::scene::Assets;
use shred::{Read, World, Write};

pub fn render_system(
    components: &World,
    assets: &Assets,
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
) {
    let system_data: (
        Read<ComponentMap<PositionComponent>>,
        Read<ComponentMap<TextureId>>,
        Read<ComponentMap<SizeComponent>>,
    ) = components.system_data();

    let (positions, textures, sizes) = system_data;

    canvas.clear();

    let data_iter = positions
        .data()
        .into_iter()
        .zip(textures.data().into_iter())
        .zip(sizes.data().into_iter());

    for ((pos, texture_id), size) in data_iter {
        match (pos, texture_id, size) {
            (Some(pos), Some(texture_id), Some(size)) => {
                let texture = assets.get(&texture_id.value.0).unwrap();
                canvas
                    .copy(
                        &texture,
                        None,
                        sdl2::rect::Rect::new(
                            pos.value.0.x as i32,
                            pos.value.0.y as i32,
                            size.value.0 as u32,
                            size.value.1 as u32,
                        ),
                    )
                    .unwrap();
            }
            (_, _, _) => {}
        }
    }

    canvas.present();
}

pub fn physics_system(components: &mut World, dt: f64, time_scale: f64) {
    let system_data: (
        Write<ComponentMap<PositionComponent>>,
        Write<ComponentMap<PhysicsComponent>>,
    ) = components.system_data();
    let mut positions = system_data.0;
    let mut physics = system_data.1;

    let data_iter = positions
        .data_mut()
        .iter_mut()
        .zip(physics.data_mut().iter_mut());

    let dt = dt * time_scale;

    for (pos, physics) in data_iter {
        match (pos, physics) {
            (Some(pos), Some(physics)) => {
                physics.value.vel += physics.value.accel * dt;
                pos.value.0 += physics.value.vel * dt;
            }
            (_, _) => {}
        }
    }
}

pub fn circular_world_system(components: &mut World, scene_size: &(f64, f64)) {
    let system_data: Write<ComponentMap<PositionComponent>> = components.system_data();
    let mut positions = system_data;

    let data = positions.data_mut();

    for pos in data.iter_mut() {
        if let Some(position) = pos {
            if position.value.0.y > scene_size.1 {
                position.value.0.y = 0.;
            }
            if position.value.0.y < 0. {
                position.value.0.y = scene_size.1 - 1.;
            }
            if position.value.0.x > scene_size.0 {
                position.value.0.x = 0.;
            }
            if position.value.0.x < 0. {
                position.value.0.x = scene_size.0;
            }
        }
    }
}

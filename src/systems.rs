use shred::{Read, World, Write};
use voxelviewer::ViewActions;
use world::components::*;

pub fn render_system(
    components: &mut World,
    view: &mut ViewActions
) {
    let system_data: (
        Read<ComponentMap<RenderComponent>>,
    ) = components.system_data();

    let renders = system_data;
    
    let data_iter = renders.0.data().into_iter();
    for render in data_iter {
        match render {
            Some(render) =>{
                view.update_cube(&render.value.obj);
            },
            _ => {}
        }
    }
}

pub fn physics_system(components: &mut World, dt: f32, time_scale: f32) {
    let system_data: (
        Write<ComponentMap<RenderComponent>>,
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
                physics.value.vel += physics.value.accel * dt as f32;
                pos.value.obj.position += physics.value.vel * dt as f32;
            }
            (_, _) => {}
        }
    }
}

pub fn circular_world_system(components: &mut World, scene_size: &(f32, f32)) {
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

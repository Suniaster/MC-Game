use std::sync::{Mutex, Arc};

use nalgebra::Point3;
use specs::prelude::*;

use crate::ScreenView;

use super::components::{LookingDirectionComponent, PositionComponent};

pub struct CameraResource {
    entity: specs::Entity,
}

pub struct CameraSystem{
}


impl CameraSystem {
    pub fn new() -> Self {
        Self {
        }
    }
}

impl<'a> System<'a> for CameraSystem {
    type SystemData = (
        WriteExpect<'a, Arc<Mutex<ScreenView>>>,
        ReadExpect<'a, CameraResource>,
        ReadStorage<'a, PositionComponent>,
        ReadStorage<'a, LookingDirectionComponent>
    );

    fn setup(&mut self, world: &mut specs::World) {
        Self::SystemData::setup(world);
        world.register::<LookingDirectionComponent>();
        
        let camera = world
            .create_entity()
            .with(LookingDirectionComponent::new(0.0, 0.0))
            .with(PositionComponent::new(Point3::new(0.0, 0.0, 0.0)))
            .build();

        world.insert(CameraResource {entity: camera});
    }

    fn run(&mut self, (screen_view, camera, pc, ld): Self::SystemData) {
        let pos = pc.get(camera.entity);
        let look_dir = ld.get(camera.entity);

        let mut screen_view = screen_view.lock().unwrap();
        if let Some(pos) = pos {
            screen_view.state.camera.position = pos.0;
        }
        if let Some(look_dir) = look_dir {
            screen_view.state.camera.yaw = look_dir.yaw;
            screen_view.state.camera.pitch = look_dir.pitch;
        }
    }
}
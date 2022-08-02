use std::sync::{Mutex};

use common::PositionComponent;
use specs::prelude::*;

use crate::scene::State;

use super::components::{LookingDirectionComponent};

pub struct CameraResource {
    pub entity: specs::Entity,
}

impl CameraResource {
    pub fn new(entity: specs::Entity) -> Self {
        Self {
            entity,
        }
    }
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
        WriteExpect<'a, Mutex<State>>,
        ReadExpect<'a, CameraResource>,
        ReadStorage<'a, PositionComponent>,
        ReadStorage<'a, LookingDirectionComponent>
    );

    fn run(&mut self, (state_mutex, camera, pc, ld): Self::SystemData) {
        let mut state = state_mutex.lock().unwrap();
        let pos = pc.get(camera.entity);
        let look_dir = ld.get(camera.entity);

        if let Some(pos) = pos {
            state.camera.position = pos.0;
        }
        if let Some(look_dir) = look_dir {
            state.camera.yaw = look_dir.yaw;
            state.camera.pitch = look_dir.pitch;
        }
    }
}
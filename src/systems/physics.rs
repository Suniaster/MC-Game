use crate::components::PhysicsComponent;
use voxelviewer::view_system::components::PositionComponent;
use specs::prelude::*;

use super::WorldDt;

pub struct PhysicsSystem;

impl<'a> System<'a> for PhysicsSystem {
    type SystemData = (
        WriteStorage<'a, PhysicsComponent>,
        WriteStorage<'a, PositionComponent>,
        ReadExpect<'a, WorldDt>
    );

    fn run(&mut self, (mut physics, mut pos, dt): Self::SystemData) {
        for (physics, pos) in (&mut physics, &mut pos).join() {
            physics.velocity += physics.acceleration * dt.0.as_secs_f32();
            pos.0 += physics.velocity * dt.0.as_secs_f32();
        }
    }
}
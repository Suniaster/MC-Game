use specs::prelude::*;

struct Camera {
    entity: specs::Entity,
}

struct CameraSystem {
}

impl CameraSystem {
    pub fn new() -> Self {
        CameraSystem {
        }
    }
}

impl<'a> System<'a> for CameraSystem {
    type SystemData = (
        ReadExpect<'a, Camera>,
    );

    fn setup(&mut self, world: &mut specs::World) {
        Self::SystemData::setup(world);

        let camera = world
            .create_entity()
            .build();

        world.insert(Camera {entity: camera});
    }

    fn run(&mut self, (camera): Self::SystemData) {
        
    }
}
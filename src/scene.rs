use super::components;

use shred::World;

use components::PhysicsComponent;
use components::*;

use ecs::ComponentVecAllocator;


pub struct GameScene {
    pub entity_allocator: ComponentVecAllocator,

    // Components
    pub components: World,

    // Resources
    pub scene_size: (f64, f64), // Width, heigth
    pub time_scale: f64,

    // Entities
    pub blobs: Vec<EntityIdx>,
}

impl GameScene {
    pub fn new(size: (f64, f64)) -> GameScene {
        return GameScene {
            entity_allocator: ComponentVecAllocator::new(),

            components: World::empty(),

            scene_size: size,
            time_scale: 5.,

            blobs: vec![],
        };
    }

    pub fn setup_components(&mut self) {
        self.components
            .insert(ComponentMap::<PositionComponent>::new());
        self.components
            .insert(ComponentMap::<PhysicsComponent>::new());
        self.components.insert(ComponentMap::<SizeComponent>::new());
        self.components.insert(ComponentMap::<TextureId>::new());
    }


    pub fn add_component<T: shred::Resource>(&mut self, idx: &EntityIdx, c: T) {
        self.components
            .get_mut::<ComponentMap<T>>()
            .unwrap()
            .set(idx, c);
    }
}

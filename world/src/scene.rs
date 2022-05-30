use std::collections::HashMap;

use super::components;

use shred::World;

use components::PhysicsComponent;
use components::*;
use ecs::{ComponentVecAllocator, vec_storage::ArrayEntry};

use core::slice::{Iter};

pub struct GameScene {
    pub entity_allocator: ComponentVecAllocator,

    // Components
    pub components: World,

    // Resources
    pub scene_size: (f32, f32), // Width, heigth
    pub time_scale: f64,
    pub loaded_terrain: HashMap<isize, bool>,

    // Entities
    pub entities: HashMap<String, Vec<EntityIdx>>,
    pub cubes: Vec<EntityIdx>,
    pub terrain_chunk: Vec<EntityIdx>
}

impl GameScene {
    pub fn new() -> GameScene {
        let mut scene = GameScene {
            entity_allocator: ComponentVecAllocator::new(),

            components: World::empty(),

            scene_size: (32.,32.),
            time_scale: 5.,
            loaded_terrain: HashMap::new(),

            entities: HashMap::new(),
            cubes: vec![],
            terrain_chunk: vec![]
        };
        scene.setup_components();
        scene
    }

    pub fn setup_components(&mut self) {
        self.components
            .insert(ComponentMap::<PositionComponent>::new());
        self.components
            .insert(ComponentMap::<PhysicsComponent>::new());
        self.components.insert(ComponentMap::<SizeComponent>::new());
        self.components.insert(ComponentMap::<RenderComponent>::new());
    }


    pub fn add_component<T: shred::Resource>(&mut self, idx: &EntityIdx, c: T) {
        self.components
            .get_mut::<ComponentMap<T>>()
            .unwrap()
            .set(idx, c);
    }

    pub fn create_entity_container(&mut self, name: &str) {
        self.entities.insert(name.to_string(), vec![]);
    }

    pub fn insert_entity(&mut self, name: &str, idx: EntityIdx) {
        self.entities
            .get_mut(name)
            .unwrap()
            .push(idx);
    }
    pub fn get_iter<T: shred::Resource>(&mut self) -> Iter<Option<ArrayEntry<T>>> {
        return self.components
            .get_mut::<ComponentMap<T>>()
            .unwrap()
            .data()
            .iter();
    }
}

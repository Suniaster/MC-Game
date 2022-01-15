use super::components;

use shred::World;
use std::collections::HashMap;

use components::PhysicsComponent;
use components::*;

use crate::sdl2::image::LoadTexture;
use crate::sdl2;
use ecs::ComponentVecAllocator;

pub type Assets = HashMap<String, sdl2::render::Texture>;

pub struct GameScene {
  pub entity_allocator:  ComponentVecAllocator,

  // Components
  pub components: World,

  // Resources
  pub scene_size: (f64, f64), // Width, heigth
  pub time_scale: f64,
  pub assets: Assets,

  // Entities
  pub blobs: Vec<EntityIdx>,
}

impl GameScene {
  pub fn new(size: (f64, f64)) -> GameScene {
    return GameScene{
      entity_allocator: ComponentVecAllocator::new(),
      
      components: World::empty(),

      assets: HashMap::new(),
      scene_size: size,
      time_scale: 5.,

      blobs: vec![],
    }
  }

  pub fn setup_components(&mut self){
    self.components.insert(ComponentMap::<PositionComponent>::new());
    self.components.insert(ComponentMap::<PhysicsComponent>::new());
    self.components.insert(ComponentMap::<SizeComponent>::new());
    self.components.insert(ComponentMap::<TextureId>::new());
  }

  pub fn setup_assets(&mut self, texture_creator: &sdl2::render::TextureCreator<sdl2::video::WindowContext>){
    self.assets.insert(String::from("test"), texture_creator.load_texture("./assets/av.jpg").expect(""));
  }

  pub fn add_component<T: shred::Resource>(&mut self, idx: &EntityIdx, c: T){
    self.components.get_mut::<ComponentMap<T>>()
      .unwrap().set(idx, c);
  }
}
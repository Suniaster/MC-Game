use super::components;

use shred::World;
use std::collections::HashMap;

use components::PhysicsComponent;
use components::*;

use crate::sdl2::image::LoadTexture;
use crate::sdl2;
use ecs::ComponentVecAllocator;

use ncollide2d::math::Vector;

pub type Assets = HashMap<String, sdl2::render::Texture>;

pub struct GameScene {
  entity_allocator:  ComponentVecAllocator,

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
  
  pub fn create_blob(&mut self){
    let blob_idx = self.entity_allocator.allocate();

    self.components.get_mut::<ComponentMap::<PositionComponent>>().unwrap().set(
      &blob_idx, PositionComponent(Vector::new(100., 100.))
    );

    self.components.get_mut::<ComponentMap::<PhysicsComponent>>().unwrap().set(
      &blob_idx, PhysicsComponent::new_random()
    );

    self.components.get_mut::<ComponentMap::<SizeComponent>>().unwrap().set(
      &blob_idx, SizeComponent(40., 40.)
    );

    self.components.get_mut::<ComponentMap::<TextureId>>().unwrap().set(
      &blob_idx, TextureId(String::from("test"))
    );

    self.blobs.push(blob_idx);
  }
}
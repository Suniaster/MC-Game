use super::components;

use components::PhysicsComponent;
use components::*;

use crate::sdl2::image::LoadTexture;
use crate::sdl2;
use ecs::ComponentVecAllocator;

use ncollide2d::math::Vector;

pub struct GameScene {
  entity_allocator:  ComponentVecAllocator,

  // Components
  pub positions: ComponentMap<PositionComponent>,
  pub physics: ComponentMap<PhysicsComponent>,
  pub textures: ComponentMap<sdl2::render::Texture>,
  pub sizes:  ComponentMap<SizeComponent>,

  // Resources
  pub scene_size: (f64, f64), // Width, heigth
  pub time_scale: f64,

  // Entities
  pub blobs: Vec<EntityIdx>,
}


impl GameScene {
  pub fn new(size: (f64, f64)) -> GameScene {
    return GameScene{
      entity_allocator: ComponentVecAllocator::new(),
      
      positions: ComponentMap::new(),
      physics: ComponentMap::new(), 
      textures: ComponentMap::new(),
      sizes: ComponentMap::new(),

      scene_size: size,
      time_scale: 5.,

      blobs: vec![],
    }
  }

  pub fn create_blob(&mut self, texture_creator: &sdl2::render::TextureCreator<sdl2::video::WindowContext>){
    let blob_idx = self.entity_allocator.allocate();
    let texture = texture_creator.load_texture("./assets/av.jpg").expect("");

    self.positions.set(&blob_idx, PositionComponent(Vector::new(100., 100.)));
    self.textures.set(&blob_idx, texture);
    self.physics.set(&blob_idx, PhysicsComponent::new_random());
    self.sizes.set(&blob_idx, SizeComponent(40., 40.));

    self.blobs.push(blob_idx);
  }

  pub fn create_enemy(&mut self, texture_creator: &sdl2::render::TextureCreator<sdl2::video::WindowContext>){
    let blob_idx = self.entity_allocator.allocate();
    let texture = texture_creator.load_texture("./assets/buizel.png").expect("");

    self.positions.set(&blob_idx, PositionComponent(Vector::new(100.0, 100.0)));
    self.textures.set(&blob_idx, texture);
    self.sizes.set(&blob_idx, SizeComponent(90., 90.));

    self.blobs.push(blob_idx);
  }
}
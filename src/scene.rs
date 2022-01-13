use super::components;

use components::PhysicsComponent;
use components::*;

use crate::sdl2::image::LoadTexture;
use crate::sdl2;
use ecs::ComponentVecAllocator;

use sdl2::rect;
use ncollide2d::math::Vector;

pub struct GameScene {
  entity_allocator:  ComponentVecAllocator,

  // Components
  pub positions: ComponentMap<PositionComponent>,
  pub physics: ComponentMap<PhysicsComponent>,
  pub textures: ComponentMap<sdl2::render::Texture>,
  pub sizes:  ComponentMap<(f64, f64)>,

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
    self.sizes.set(&blob_idx, (40., 40.));

    self.blobs.push(blob_idx);
  }

  pub fn create_enemy(&mut self, texture_creator: &sdl2::render::TextureCreator<sdl2::video::WindowContext>){
    let blob_idx = self.entity_allocator.allocate();
    let texture = texture_creator.load_texture("./assets/buizel.png").expect("");

    self.positions.set(&blob_idx, PositionComponent(Vector::new(100.0, 100.0)));
    self.textures.set(&blob_idx, texture);
    self.sizes.set(&blob_idx, (90., 90.));

    self.blobs.push(blob_idx);
  }

  pub fn render_system(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>){
    let data_iter = self.positions.data().into_iter()
      .zip(self.textures.data().into_iter())
      .zip(self.sizes.data().into_iter());

    for ((pos, texture), size) in data_iter {
      let pos = pos.as_ref().unwrap();
      let texture = texture.as_ref().unwrap();
      let size = size.as_ref().unwrap();

      let r = rect::Rect::new(
        pos.value.0.x as i32, 
        pos.value.0.y as i32, 
        size.value.0 as u32, 
        size.value.0 as u32
      );
      canvas.copy(&texture.value, None, r).unwrap();
    }
  }
}

pub fn physics_system(
  position: &mut ComponentMap<PositionComponent>,
  physics: &mut ComponentMap<PhysicsComponent>,
  dt: f64,
  time_scale: f64
){
  let data_iter = position.data_mut().iter_mut().zip(
    physics.data_mut().iter_mut()
  );
  let dt = dt * time_scale;

  for (pos, physics) in data_iter {
    match (pos, physics) {
      (Some(pos), Some(physics)) => {
        physics.value.vel += physics.value.accel * dt;
        pos.value.0 += physics.value.vel * dt;
      },
      (_, _) =>{}
    }
  }
}

pub fn circular_world_system(positions: &mut ComponentMap<PositionComponent>, scene_size: &(f64, f64)){
  let data = positions.data_mut();

  for pos in data.iter_mut(){
    if let Some(position) = pos{
      if position.value.0.y > scene_size.1{
        position.value.0.y = 0.;
      }
      if position.value.0.y < 0. {
        position.value.0.y = scene_size.1 - 1.;
      }
      if position.value.0.x > scene_size.0{
        position.value.0.x = 0.;
      }
      if position.value.0.x < 0.{
        position.value.0.x = scene_size.0;
      }
    }
  }
}
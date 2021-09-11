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
  positions: ComponentMap<Vector<f64>>,
  physics: ComponentMap<PhysicsComponent>,
  textures: ComponentMap<sdl2::render::Texture>,
  sizes:  ComponentMap<(f64, f64)>,

  // Entities
  blobs: Vec<EntityIdx>,
}


impl GameScene {
  pub fn new() -> GameScene {
    return GameScene{
      entity_allocator: ComponentVecAllocator::new(),
      
      positions: ComponentMap::new(),
      physics: ComponentMap::new(), 
      textures: ComponentMap::new(),
      sizes: ComponentMap::new(),

      blobs: vec![],
    }
  }

  pub fn create_blob(&mut self, texture_creator: &sdl2::render::TextureCreator<sdl2::video::WindowContext>){
    let blob_idx = self.entity_allocator.allocate();
    let texture = texture_creator.load_texture("./assets/av.jpg").expect("");

    self.positions.set(&blob_idx, Vector::new(30.0, 30.0));
    self.textures.set(&blob_idx, texture);
    self.physics.set(&blob_idx, PhysicsComponent::new());
    self.sizes.set(&blob_idx, (40., 40.));

    self.blobs.push(blob_idx);
  }

  pub fn create_enemy(&mut self, texture_creator: &sdl2::render::TextureCreator<sdl2::video::WindowContext>){
    let blob_idx = self.entity_allocator.allocate();
    let texture = texture_creator.load_texture("./assets/buizel.png").expect("");

    self.positions.set(&blob_idx, Vector::new(100.0, 100.0));
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
        pos.value.x as i32, 
        pos.value.y as i32, 
        size.value.0 as u32, 
        size.value.0 as u32
      );
      canvas.copy(&texture.value, None, r).unwrap();
    }
  }

  pub fn physics_system(&mut self, dt:f64){
    let data_iter = self.positions.data_mut().iter_mut()
      .zip(self.physics.data_mut().iter_mut());

    for (pos, physics) in data_iter {
      let pos = pos.as_mut().unwrap();
      let physics = physics.as_mut().unwrap();

      physics.value.vel += physics.value.accel *dt;
      pos.value += physics.value.vel * dt;
    }
  }
}


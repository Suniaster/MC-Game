use super::components::*;


pub fn render_system(
  canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
  positions: &ComponentMap<PositionComponent>,
  textures: &ComponentMap<sdl2::render::Texture>,
  sizes: &ComponentMap<SizeComponent>
){
  canvas.clear();

  let data_iter = positions.data().into_iter()
    .zip(textures.data().into_iter())
    .zip(sizes.data().into_iter());
  
  for ((pos, texture), size) in data_iter{
    match (pos, texture, size){
      (Some(pos), Some(texture), Some(size))=>{
        canvas.copy(&texture.value, None, 
          sdl2::rect::Rect::new(
            pos.value.0.x as i32,
            pos.value.0.y as i32, 
            size.value.0 as u32, 
            size.value.0 as u32
          )
        ).unwrap();
      },
      (_, _, _)=>{}
    }
  }
  
  canvas.present();
}

pub fn physics_system(
  positions: &mut ComponentMap<PositionComponent>,
  physics: &mut ComponentMap<PhysicsComponent>,
  dt: f64,
  time_scale: f64
){
  let data_iter = positions.data_mut().iter_mut().zip(
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
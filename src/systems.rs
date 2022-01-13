use super::components::*;

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
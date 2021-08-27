use super::world::HasWorldInfo;

pub trait Physics2D {
  fn move_dt(&mut self, dt: f64) ;
  fn set_pos(&mut self, x:f64, y:f64);
}

impl<T: HasWorldInfo> Physics2D for T{
  fn set_pos(&mut self, x:f64, y:f64){
      let info = self.get_mut_world();
      info.pos.x = x;
      info.pos.y = y;
  }

  fn move_dt(&mut self, dt: f64){
      let info = self.get_mut_world();
      info.pos.x += info.vel.x * dt;
      info.pos.y += info.vel.y * dt;
  }
}
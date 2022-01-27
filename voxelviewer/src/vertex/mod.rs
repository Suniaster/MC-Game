pub mod static_vertex;

use cgmath::Vector3;
use cgmath::InnerSpace;
use static_vertex::StaticVertex;

pub type VertPos = Vector3<f32>;

pub struct StaticVertexMesh{
  pub vertices: Vec<StaticVertex>,
  pub position: cgmath::Vector3<f32>,
}

impl StaticVertexMesh {
  pub fn update_pos(&mut self, new_pos: VertPos)->bool{
    let disloc = new_pos - self.position;
    self.position = new_pos;
    if disloc.magnitude() == 0.{return false}
    for v in self.vertices.iter_mut(){
      v.position = (cgmath::Vector3::from(v.position) + disloc).into();
    }
    true
  }

  pub fn to_buffer<T: bytemuck::Pod>(&self)->&[T]{
    bytemuck::cast_slice::<StaticVertex, T>(&self.vertices)
  }
}
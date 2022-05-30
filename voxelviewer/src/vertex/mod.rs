pub mod static_vertex;
use static_vertex::StaticVertex;

use nalgebra::Vector3;
use nalgebra::Point3;

pub struct StaticVertexMesh{
  pub vertices: Vec<StaticVertex>,
  pub position: Point3<f32>,
}

impl StaticVertexMesh {
  pub fn update_pos(&mut self, new_pos: Point3<f32>)->bool{
    let disloc = new_pos - self.position;
    self.position = new_pos;
    if disloc.magnitude() == 0.{return false}
    for v in self.vertices.iter_mut(){
      v.position = (Vector3::from(v.position) + disloc).into();
    }
    true
  }

  pub fn to_buffer<T: bytemuck::Pod>(&self)->&[T]{
    bytemuck::cast_slice::<StaticVertex, T>(&self.vertices)
  }

  pub fn get_indices_for_square_mesh(&self)->Vec<u32>{
    let mut indices:Vec<u32> = vec![];
    let num_faces = self.vertices.len() / 4;
    for i in 0..(num_faces){
      let disloc = (i*4) as u32;
      indices.push(2 + disloc as u32);
      indices.push(1 + disloc as u32);
      indices.push(0 + disloc as u32);

      indices.push(3 + disloc as u32);
      indices.push(2 + disloc as u32);
      indices.push(0 + disloc as u32);
    }
    indices
  }

  pub fn new_empty() -> Self{
    Self{
      vertices: vec![], position: Point3::origin()
    }
  }
}
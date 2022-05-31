pub mod static_vertex;
pub mod mesh_instance;
use static_vertex::StaticVertex;
use mesh_instance::MeshInstance;

pub struct StaticVertexMesh{
  pub vertices: Vec<StaticVertex>,
  pub mesh_instance: [MeshInstance; 1],

}

impl StaticVertexMesh {
  pub fn new(vertices: Vec<StaticVertex>, position: [f32;3]) -> Self {
    Self {
      vertices,
      mesh_instance: [MeshInstance { position }]
    }
  }

  pub fn update_pos(&mut self, new_pos: [f32;3])->bool{
    if self.mesh_instance[0].position == new_pos{
      return false;
    }
    self.mesh_instance[0].position = new_pos;
    true
  }

  pub fn to_vertex_buffer<T: bytemuck::Pod>(&self)->&[T]{
    bytemuck::cast_slice::<StaticVertex, T>(&self.vertices)
  }

  pub fn to_instance_buffer<T: bytemuck::Pod>(&self)->&[T]{
    bytemuck::cast_slice::<MeshInstance, T>(&self.mesh_instance)
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
    return Self::new(vec![], [0.,0.,0.]);
  }
}
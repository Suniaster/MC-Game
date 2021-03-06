pub mod vertex;
pub mod instance;
use vertex::Vertex;
use instance::MeshInstance;

pub struct StaticVertexMesh{
  pub vertices: Vec<Vertex>,
  pub mesh_instance: [MeshInstance; 1],
}

impl StaticVertexMesh {
  pub fn new(vertices: Vec<Vertex>, origin: [f32;3]) -> Self {
    Self {
      vertices,
      mesh_instance: [MeshInstance { origin }]
    }
  }

  pub fn update_origin(&mut self, new_origin: [f32;3])->bool{
    if self.mesh_instance[0].origin == new_origin{
      return false;
    }
    self.mesh_instance[0].origin = new_origin;
    true
  }

  pub fn to_vertex_buffer<T: bytemuck::Pod>(&self)->&[T]{
    bytemuck::cast_slice::<Vertex, T>(&self.vertices)
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
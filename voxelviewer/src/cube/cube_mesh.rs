use super::super::vertex::static_vertex::StaticVertex;
use super::super::voxel;
use cgmath::InnerSpace;

pub struct CubeMesh{
  pub vertices: Vec<StaticVertex>,
  position: cgmath::Vector3<f32>,
  half_size: f32
}

impl CubeMesh{
  pub fn to_buffer<T: bytemuck::Pod>(&self)->&[T]{
    bytemuck::cast_slice::<StaticVertex, T>(&self.vertices)
  }

  pub fn new(half_size: f32) -> Self {
    let quad = voxel::DefaultQuad::new();
    let vertexes = quad.get_complete_vertexes();

    let vs = vertexes.iter().map(|v|{
      StaticVertex{
        color_diffuse: [0.1, 0.2, 0.5],
        position: v.position,
        normal: v.normal
      }
    }).collect::<Vec<_>>();
    Self{vertices: vs, half_size:0.5, position: cgmath::Vector3::new(0., 0., 0.)}
  }

  pub fn update_pos(&mut self, new_pos: cgmath::Vector3<f32>)->bool{
    let disloc = new_pos - self.position;
    self.position = new_pos;
    if disloc.magnitude() == 0.{return false}
    for v in self.vertices.iter_mut(){
      v.position = (cgmath::Vector3::from(v.position) + disloc).into();
    }
    true
  }
}
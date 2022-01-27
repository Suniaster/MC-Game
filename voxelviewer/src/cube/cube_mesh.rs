use super::super::vertex::{
  StaticVertexMesh
};
use super::super::quad::hexagon;

pub fn new_cube(half_size: f32) -> StaticVertexMesh {
  let cube = hexagon::HexagonMesh::new(
    cgmath::Vector3::new(0.,0.,0.), 
    cgmath::Vector3::new(half_size,half_size,half_size)
  );

  StaticVertexMesh{
    vertices: cube.get_static_vertices(),
    position: cube.center_position
  }
}
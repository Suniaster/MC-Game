use super::vertex::{
  StaticVertexMesh, StaticVertexBuild
};
use super::quad::hexagon;

pub fn _new_cube(half_size: f32) -> StaticVertexMesh {
  let cube = hexagon::HexagonMesh::new(
    cgmath::Vector3::new(0.,0.,0.), 
    cgmath::Vector3::new(half_size,half_size,half_size),
    [0.1, 1.0, 0.1]
  );
  cube.build()
}
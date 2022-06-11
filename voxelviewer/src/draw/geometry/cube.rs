use crate::{draw::mesh::{StaticVertexMesh, vertex::Vertex}, geometry::cube::Cuboid};

use super::cube_face::cube_face_to_vertex_list;


pub fn get_cube_vertices(cube: &Cuboid) -> Vec<Vertex>{
  let mut result = vec![];
  for quad in &cube.faces {
      let mut vertices = cube_face_to_vertex_list(&quad);
      result.append(&mut vertices);
  }
  result
}

pub fn build_cube_mesh(cube: &Cuboid) -> StaticVertexMesh{
  return StaticVertexMesh::new(
    get_cube_vertices(cube), 
    cube.origin.into()
  );
}
use nalgebra::{Point3, Vector3};

use crate::{draw::mesh::StaticVertexMesh, geometry::{cube::Cuboid, grid::{Grid, CubeTensor}}};

use super::cube_face::cube_face_to_vertex_list;


type Mat3d<T> = Vec<Vec<Vec<T>>>;

pub fn build_grid_mesh_from_desc(color: [f32;3], tensor: &CubeTensor) -> StaticVertexMesh {
    let mut hexes:Vec<Cuboid> = vec![];
    let positions = tensor.get_positions();

    for pos in positions {
      let mut cube = Cuboid::new(Vector3::new(
        tensor.cube_half_size, tensor.cube_half_size, tensor.cube_half_size
      ), color);
      Grid::filter_cube(&tensor.desc, &mut cube, pos.1[0], pos.1[1], pos.1[2]);
      cube.move_vertices(&pos.0.translation.vector);
      hexes.push(cube);
    }

    return build_grid_mesh_from_cube_matrix(Point3::origin(), &hexes);
}

pub fn build_grid_mesh_from_cube_matrix(pos: Point3<f32>, cube_mat: &Vec<Cuboid>) -> StaticVertexMesh{
  let mut vertices = vec![];
    for hex in cube_mat{
      for quad in &hex.faces {
        let mut quad_vertices = cube_face_to_vertex_list(&quad);
        vertices.append(&mut quad_vertices);
      } 
    }
    return StaticVertexMesh::new(
        vertices, 
        pos.into()
    );
}
use nalgebra::Point3;

use crate::{draw::mesh::StaticVertexMesh, geometry::{cube::Cuboid, grid::Grid}};

use super::cube_face::cube_face_to_vertex_list;


type Mat3d<T> = Vec<Vec<Vec<T>>>;

pub fn build_grid_mesh_from_desc(grid: &Grid, desc: &Mat3d<bool>) -> StaticVertexMesh {
    let mut hexes:Mat3d<Cuboid> = vec![];

    for x_idx in 0..desc.len(){
      hexes.push(vec![]);
      for y_idx in 0..desc[x_idx].len(){
        hexes[x_idx].push(vec![]);
        for z_idx in 0..desc[x_idx][y_idx].len(){
          if desc[x_idx][y_idx][z_idx] {
            let cube = grid.create_hex_in_pos(desc, x_idx, y_idx, z_idx);
            if !cube.is_empty(){
              hexes[x_idx][y_idx].push(cube);
            }
          }
        }
      }
    }

    return build_grid_mesh_from_cube_matrix(grid.origin, &hexes);
}

pub fn build_grid_mesh_from_cube_matrix(pos: Point3<f32>, cube_mat: &Mat3d<Cuboid>) -> StaticVertexMesh{
  let mut vertices = vec![];
    for row in cube_mat{
        for col in row{
            for hex in col{
                for quad in &hex.faces {
                    let mut quad_vertices = cube_face_to_vertex_list(&quad);
                    vertices.append(&mut quad_vertices);
                }
            }
        }
    }
    return StaticVertexMesh::new(
        vertices, 
        pos.into()
    );
}
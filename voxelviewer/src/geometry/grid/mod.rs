use crate::cube;

use super::super::vertex::{StaticVertexMesh};
use super::cube::Cuboid;
use nalgebra::{Point3, Vector3};

use super::cube_face::cube_face_direction::CubeFaceDirection;

const GRID_SIZE: usize = 16;

pub type GridMatrix =  Vec<Vec<Vec<bool>>>;
pub struct Grid{
  pub position: Point3<f32>,
  pub cube_size: f32,
  pub cube_grid: GridMatrix
}


impl Grid{
  pub fn _create_empty(position: [f32;3], cube_size: f32, grid_size: [usize;3]) -> Self{
    let cube_grid: GridMatrix = vec![vec![vec![false;grid_size[2]];grid_size[1]];grid_size[0]];
    Self { 
      position: Point3::from(position), 
      cube_size, 
      cube_grid
    }
  }

  pub fn create_from(position: [f32;3], cube_size: f32, grid_matrix: GridMatrix) -> Self{
    Self { 
      position: Point3::from(position), 
      cube_size, 
      cube_grid: grid_matrix
    }
  }

  pub fn _map_grid(&mut self, func: fn(&mut GridMatrix, usize, usize, usize)){
    for i in 0..self.cube_grid.len(){
      for j in 0..self.cube_grid[i].len(){
        for k in 0..self.cube_grid[i][j].len(){
          func(&mut self.cube_grid, i, j, k);
        }
      }
    }
  }

  fn create_hex_in_pos(&self, i:usize, j:usize, k:usize)->Cuboid{
    let grid_half_size = GRID_SIZE as f32*self.cube_size  / 2.;
    let mut pos = self.position - Vector3::from([grid_half_size, grid_half_size, grid_half_size]);
    pos.x += i as f32 * self.cube_size;
    pos.y += j as f32 * self.cube_size;
    pos.z += k as f32 * self.cube_size;

    let hex = Cuboid::new(
      pos + Vector3::new(self.cube_size/2., self.cube_size/2., self.cube_size/2.),
      Vector3::new(self.cube_size/2., self.cube_size/2., self.cube_size/2.),
      [0.1, 1.0, 0.1]
    );

    return hex;
  }

  pub fn filter_cube(&self, cube: &mut Cuboid, grid_pos: [usize; 3]){
 
    if grid_pos[1] < GRID_SIZE - 1{
      if self.cube_grid[grid_pos[0]][grid_pos[1] + 1][grid_pos[2]]{
        cube.remove_face(CubeFaceDirection::Up);
      }
    }
    if grid_pos[1] > 0{
      if self.cube_grid[grid_pos[0]][grid_pos[1] - 1][grid_pos[2]]{
        cube.remove_face(CubeFaceDirection::Down);
      }
    }

    if grid_pos[2] < GRID_SIZE - 1{
      if self.cube_grid[grid_pos[0]][grid_pos[1]][grid_pos[2] + 1]{
        cube.remove_face(CubeFaceDirection::Back);
      }
    }
    if grid_pos[2] > 0{
      if self.cube_grid[grid_pos[0]][grid_pos[1]][grid_pos[2] - 1]{
        cube.remove_face(CubeFaceDirection::Front);
      }
    }

    if grid_pos[0] < GRID_SIZE - 1{
      if self.cube_grid[grid_pos[0] + 1][grid_pos[1]][grid_pos[2]]{
        cube.remove_face(CubeFaceDirection::Right);
      }
    }

    if grid_pos[0] > 0{
      if self.cube_grid[grid_pos[0] - 1][grid_pos[1]][grid_pos[2]]{
        cube.remove_face(CubeFaceDirection::Left);
      }
    }
  }


  pub fn build(&self)->StaticVertexMesh{
    let mut hexes:Vec<Vec<Vec<Cuboid>>> = vec![];
    let (max_x, max_y, max_z) = (
      self.cube_grid[0].len(), 
      self.cube_grid[1].len(),
      self.cube_grid[2].len()
    );

    for x_idx in 0..max_x{
      hexes.push(vec![]);
      for y_idx in 0..max_y{
        hexes[x_idx].push(vec![]);
        for z_idx in 0..max_z{
          if self.cube_grid[x_idx][y_idx][z_idx] {
            let mut cube = self.create_hex_in_pos(x_idx, y_idx, z_idx);
            self.filter_cube(&mut cube, [x_idx, y_idx, z_idx]);
            hexes[x_idx][y_idx].push(cube);
          }
        }
      }
    }


    let mut mesh = Cuboid::build_from_grid(self.position, &hexes);
    mesh.update_pos(self.position);
    return mesh;
  }

  pub fn _build_outline(&self) -> StaticVertexMesh{
    let half_grid_size = (GRID_SIZE as f32) / 2.;
    let half_size = self.cube_size * half_grid_size;
    let overall_cuboid = cube::Cuboid::new(
      self.position, 
      Vector3::from([half_size, half_size, half_size]),
      [0., 0., 0.]
    );
    overall_cuboid.build_outline()
  }
}
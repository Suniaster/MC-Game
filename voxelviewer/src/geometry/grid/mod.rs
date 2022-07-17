use super::cube::Cuboid;
use nalgebra::{Vector3, Isometry3};

use super::cube_face::cube_face_direction::CubeFaceDirection;

const GRID_SIZE: usize = 16;

pub type GridMatrix =  Vec<Vec<Vec<bool>>>;
pub struct Grid;
impl Grid{
  pub fn filter_cube(grid: &GridMatrix, cube: &mut Cuboid, i: usize, j:usize, k:usize){
    if j < GRID_SIZE - 1{
      if grid[i][j + 1][k]{
        cube.remove_face(CubeFaceDirection::Up);
      }
    }
    if j > 0{
      if grid[i][j - 1][k]{
        cube.remove_face(CubeFaceDirection::Down);
      }
    }

    if k < GRID_SIZE - 1{
      if grid[i][j][k + 1]{
        cube.remove_face(CubeFaceDirection::Back);
      }
    }
    if k > 0{
      if grid[i][j][k - 1]{
        cube.remove_face(CubeFaceDirection::Front);
      }
    }

    if i < GRID_SIZE - 1{
      if grid[i + 1][j][k]{
        cube.remove_face(CubeFaceDirection::Right);
      }
    }

    if i > 0{
      if grid[i - 1][j][k]{
        cube.remove_face(CubeFaceDirection::Left);
      }
    }
  }
}


pub struct CubeTensor{
  pub desc: Vec<Vec<Vec<bool>>>,
  pub cube_half_size: f32,
  pub shape: [usize; 3]
}

impl CubeTensor {
  pub fn new(desc: Vec<Vec<Vec<bool>>>, cube_size: f32, shape: [usize;3]) -> Self{
      return Self{
          desc,
          cube_half_size: cube_size / 2.0,
          shape: shape
      };
  }

  pub fn get_positions(&self) -> Vec<(Isometry3<f32>, [usize;3])> {
      let mut positions:  Vec<(Isometry3<f32>, [usize;3])> = Vec::new();
      let half_tensor_sizes = Vector3::new(
          self.shape[0] as f32 * self.cube_half_size,
          self.shape[1] as f32 * self.cube_half_size,
          self.shape[2] as f32 * self.cube_half_size
      );
      let v_cube_half = Vector3::new(self.cube_half_size, self.cube_half_size, self.cube_half_size);
      let adjust_vector =  - half_tensor_sizes + v_cube_half;

      for i in 0..self.shape[0] {
          for j in 0..self.shape[1] {
              for k in 0..self.shape[2] {
                  if self.desc[i][j][k] {
                      let position_v = Vector3::new(
                          i as f32 * self.cube_half_size,
                          j as f32 * self.cube_half_size,
                          k as f32 * self.cube_half_size
                      ) + adjust_vector;

                      positions.push((Isometry3::new(
                          position_v,
                          Vector3::y()
                      ), [i, j, k]));
                  }
              }
          }
      }
      return positions;
  }
}
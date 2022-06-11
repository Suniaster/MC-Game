use super::cube::Cuboid;
use nalgebra::{Point3, Vector3};

use super::cube_face::cube_face_direction::CubeFaceDirection;

const GRID_SIZE: usize = 16;

pub type GridMatrix =  Vec<Vec<Vec<bool>>>;
pub struct Grid{
  pub origin: Point3<f32>,
  pub cube_sizes: Vector3<f32>,
  cube_half_sizes: Vector3<f32>,
  half_grid_sizes: Vector3<f32>,
  adjust_vector: Vector3<f32>
}


impl Grid{
  pub fn create_with(cube_size: f32) -> Self{
    let cube_sizes = Vector3::new(cube_size, cube_size, cube_size);
    let half_grid_sizes = Vector3::from([GRID_SIZE as f32, GRID_SIZE as f32, GRID_SIZE as f32]).component_mul(&cube_sizes) / 2.;
    let cube_half_sizes = cube_sizes/2.;
    Self { 
      origin: Point3::<f32>::origin(), 
      cube_sizes,
      half_grid_sizes,
      cube_half_sizes,
      adjust_vector: - half_grid_sizes + cube_half_sizes
    }
  }

  pub fn move_origin(&mut self, new_origin: Point3<f32>){
    self.origin = new_origin;
  }

  pub fn create_hex_in_pos(&self, grid: &GridMatrix, i:usize, j:usize, k:usize)->Cuboid{
    let mut hex = Cuboid::new(
      self.cube_half_sizes,
      [0.1, 1.0, 0.1]
    );
    Grid::filter_cube(grid, &mut hex, i, j, k);
    if hex.is_empty() {return hex;}

    let mat_world_position = self.cube_sizes.component_mul(
      &Vector3::<f32>::from([i as f32, j as f32, k as f32])
    );

    let final_pos = mat_world_position + self.adjust_vector;

    hex.move_vertices(&final_pos);
    return hex;
  }

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
use crate::cube;

use super::vertex::{StaticVertexMesh};
use super::cube::Cuboid;
use cgmath::Vector3;
use rand::prelude::*;

const GRID_SIZE: usize = 32;

pub struct Grid{
  position: cgmath::Vector3<f32>,
  cube_size: f32,
  cube_grid: [[[bool; GRID_SIZE]; GRID_SIZE]; GRID_SIZE]
}


impl Grid{
  pub fn default_empty() -> Self{
    Self{
      position: cgmath::Vector3::new(0., 0., 0.),
      cube_size: 0.2,
      cube_grid: [[[false; GRID_SIZE]; GRID_SIZE]; GRID_SIZE]
    }
  }

  pub fn default_random() -> Self{
    let mut grid = Self::default_empty();
    let mut rng = rand::thread_rng();
    let (max_x, max_y, max_z) = (
      grid.cube_grid[0].len(), 
      grid.cube_grid[1].len(),
      grid.cube_grid[2].len()
    );

    for i in 0..max_x{
      for j in 0..max_y{
        for k in 0..max_z{
          let r:f32 = rng.gen();
          grid.cube_grid[i][j][k] = r > 0.5;
        }
      }
    }

    return grid;
  }

  fn create_hex_in_pos(&self, i:usize, j:usize, k:usize)->Cuboid{
    let grid_half_size = GRID_SIZE as f32*self.cube_size  / 2.;
    let mut pos = self.position - Vector3::from([grid_half_size, grid_half_size, grid_half_size]);
    pos.x += i as f32 * self.cube_size;
    pos.y += j as f32 * self.cube_size;
    pos.z += k as f32 * self.cube_size;

    let hex = Cuboid::new(
      pos + cgmath::Vector3::new(self.cube_size/2., self.cube_size/2., self.cube_size/2.),
      cgmath::Vector3::new(self.cube_size/2., self.cube_size/2., self.cube_size/2.),
      [0.1, 1.0, 0.1]
    );

    return hex;
  }

  pub fn build(&self)->StaticVertexMesh{
    let mut hexes:Vec<Cuboid> = vec![];
    let (max_x, max_y, max_z) = (
      self.cube_grid[0].len(), 
      self.cube_grid[1].len(),
      self.cube_grid[2].len()
    );

    for x_idx in 0..max_x{
      for y_idx in 0..max_y{
        for z_idx in 0..max_z{
          if self.cube_grid[x_idx][y_idx][z_idx] {
            hexes.push(self.create_hex_in_pos(x_idx, y_idx, z_idx));
          }
        }
      }
    }
    let mut mesh = Cuboid::build_from_array(self.position, &hexes);
    mesh.update_pos(self.position);
    return mesh;
  }

  pub fn build_outline(&self) -> StaticVertexMesh{
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
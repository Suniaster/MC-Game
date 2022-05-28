use crate::cube;

use super::vertex::{StaticVertexMesh};
use super::cube::Cuboid;
use cgmath::Vector3;
use rand::prelude::*;

use super::cube_face::cube_face_direction::CubeFaceDirection;

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
          grid.cube_grid[i][j][k] = r > 0.;
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
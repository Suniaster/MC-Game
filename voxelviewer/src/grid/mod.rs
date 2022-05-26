use super::vertex::{StaticVertexMesh, StaticVertexBuild};
use super::quad::hexagon::HexagonMesh;
use rand::prelude::*;

const GRID_SIZE: usize = 32;

pub struct GridMesh{
  position: cgmath::Vector3<f32>,
  cube_size: f32,
  cube_grid: [[[bool; GRID_SIZE]; GRID_SIZE]; GRID_SIZE]
}


impl GridMesh{
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

  fn create_hex_in_pos(&self, i:usize, j:usize, k:usize)->HexagonMesh{
    let mut pos = self.position;
    pos.x += i as f32 * self.cube_size;
    pos.y += j as f32 * self.cube_size;
    pos.z += k as f32 * self.cube_size;

    let hex = HexagonMesh::new(pos,
      cgmath::Vector3::new(self.cube_size/2., self.cube_size/2., self.cube_size/2.),
      [0.1, 1.0, 0.1]
    );

    return hex;
  }

}

impl StaticVertexBuild for GridMesh {
  fn build(&self)->StaticVertexMesh{
    let mut hexes:Vec<HexagonMesh> = vec![];
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
    HexagonMesh::build_from_array(self.position, &hexes)
  }
}
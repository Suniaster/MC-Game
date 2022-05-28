
use super::{VertPos, VectorDir};
use super::Quad;

#[derive(PartialEq, Eq)]
pub enum CubeFaceDirection{
  Front, Back, Left, Right, Up, Down
}


impl CubeFaceDirection{
  pub fn quad_from_dir(dir: &CubeFaceDirection, center: &VertPos, half_sizes: &VectorDir, color: [f32; 3])->Quad{
    // let color = [0.1, 1.0, 0.1];
    /*** 1        0  
     *   * ------ *
     *   |        |
     *   |        |
     *   *--------*
     *   2        3
     */  
    match dir{
      CubeFaceDirection::Front => {
        Quad{
          vertices: [
            VertPos::new(center.x + half_sizes.x, center.y + half_sizes.y, center.z - half_sizes.z),
            VertPos::new(center.x - half_sizes.x, center.y + half_sizes.y, center.z - half_sizes.z),
            VertPos::new(center.x - half_sizes.x, center.y - half_sizes.y, center.z - half_sizes.z),
            VertPos::new(center.x + half_sizes.x, center.y - half_sizes.y, center.z - half_sizes.z),
            ],
          color,
          direction: CubeFaceDirection::Front
        }
      }
      CubeFaceDirection::Back => {
        Quad{
          vertices: [
            VertPos::new(center.x - half_sizes.x, center.y + half_sizes.y, center.z + half_sizes.z),
            VertPos::new(center.x + half_sizes.x, center.y + half_sizes.y, center.z + half_sizes.z),
            VertPos::new(center.x + half_sizes.x, center.y - half_sizes.y, center.z + half_sizes.z),
            VertPos::new(center.x - half_sizes.x, center.y - half_sizes.y, center.z + half_sizes.z),
            ],
          color,
          direction: CubeFaceDirection::Back
        }
      }
      CubeFaceDirection::Up => {
        Quad{
          vertices: [
            VertPos::new(center.x + half_sizes.x, center.y + half_sizes.y, center.z + half_sizes.z),
            VertPos::new(center.x - half_sizes.x, center.y + half_sizes.y, center.z + half_sizes.z),
            VertPos::new(center.x - half_sizes.x, center.y + half_sizes.y, center.z - half_sizes.z),
            VertPos::new(center.x + half_sizes.x, center.y + half_sizes.y, center.z - half_sizes.z),
            ],
          color,
          direction: CubeFaceDirection::Up
        }
      }
      CubeFaceDirection::Down => {
        Quad{
          vertices: [
            VertPos::new(center.x + half_sizes.x, center.y - half_sizes.y, center.z - half_sizes.z),
            VertPos::new(center.x - half_sizes.x, center.y - half_sizes.y, center.z - half_sizes.z),
            VertPos::new(center.x - half_sizes.x, center.y - half_sizes.y, center.z + half_sizes.z),
            VertPos::new(center.x + half_sizes.x, center.y - half_sizes.y, center.z + half_sizes.z),
            ],
          color,
          direction: CubeFaceDirection::Down
        }
      }
      CubeFaceDirection::Left => {
        Quad{
          vertices: [
            VertPos::new(center.x - half_sizes.x, center.y + half_sizes.y, center.z - half_sizes.z),
            VertPos::new(center.x - half_sizes.x, center.y + half_sizes.y, center.z + half_sizes.z),
            VertPos::new(center.x - half_sizes.x, center.y - half_sizes.y, center.z + half_sizes.z),
            VertPos::new(center.x - half_sizes.x, center.y - half_sizes.y, center.z - half_sizes.z),
            ],
          color,
          direction: CubeFaceDirection::Left
        }
      }
      CubeFaceDirection::Right => {
        Quad{
          vertices: [
            VertPos::new(center.x + half_sizes.x, center.y + half_sizes.y, center.z + half_sizes.z),
            VertPos::new(center.x + half_sizes.x, center.y + half_sizes.y, center.z - half_sizes.z),
            VertPos::new(center.x + half_sizes.x, center.y - half_sizes.y, center.z - half_sizes.z),
            VertPos::new(center.x + half_sizes.x, center.y - half_sizes.y, center.z + half_sizes.z),
            ],
          color,
          direction: CubeFaceDirection::Right
        }
      }
    }
  }
}
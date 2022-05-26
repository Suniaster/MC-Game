
use super::{VertPos, VectorDir};
use super::Quad;

#[derive(PartialEq, Eq)]
pub enum QuadDirection{
  Front, Back, Left, Right, Up, Down
}


impl QuadDirection{
  pub fn quad_from_dir(dir: &QuadDirection, center: &VertPos, half_sizes: &VectorDir, color: [f32; 3])->Quad{
    // let color = [0.1, 1.0, 0.1];
    match dir{
      QuadDirection::Front => {
        Quad{
          vertices: [
            VertPos::new(center.x + half_sizes.x, center.y + half_sizes.y, center.z - half_sizes.z),
            VertPos::new(center.x - half_sizes.x, center.y + half_sizes.y, center.z - half_sizes.z),
            VertPos::new(center.x - half_sizes.x, center.y - half_sizes.y, center.z - half_sizes.z),
            VertPos::new(center.x + half_sizes.x, center.y - half_sizes.y, center.z - half_sizes.z),
            ],
          color,
          direction: QuadDirection::Front
        }
      }
      QuadDirection::Back => {
        Quad{
          vertices: [
            VertPos::new(center.x - half_sizes.x, center.y + half_sizes.y, center.z + half_sizes.z),
            VertPos::new(center.x + half_sizes.x, center.y + half_sizes.y, center.z + half_sizes.z),
            VertPos::new(center.x + half_sizes.x, center.y - half_sizes.y, center.z + half_sizes.z),
            VertPos::new(center.x - half_sizes.x, center.y - half_sizes.y, center.z + half_sizes.z),
            ],
          color,
          direction: QuadDirection::Back
        }
      }
      QuadDirection::Up => {
        Quad{
          vertices: [
            VertPos::new(center.x + half_sizes.x, center.y + half_sizes.y, center.z + half_sizes.z),
            VertPos::new(center.x - half_sizes.x, center.y + half_sizes.y, center.z + half_sizes.z),
            VertPos::new(center.x - half_sizes.x, center.y + half_sizes.y, center.z - half_sizes.z),
            VertPos::new(center.x + half_sizes.x, center.y + half_sizes.y, center.z - half_sizes.z),
            ],
          color,
          direction: QuadDirection::Up
        }
      }
      QuadDirection::Down => {
        Quad{
          vertices: [
            VertPos::new(center.x + half_sizes.x, center.y - half_sizes.y, center.z - half_sizes.z),
            VertPos::new(center.x - half_sizes.x, center.y - half_sizes.y, center.z - half_sizes.z),
            VertPos::new(center.x - half_sizes.x, center.y - half_sizes.y, center.z + half_sizes.z),
            VertPos::new(center.x + half_sizes.x, center.y - half_sizes.y, center.z + half_sizes.z),
            ],
          color,
          direction: QuadDirection::Down
        }
      }
      QuadDirection::Left => {
        Quad{
          vertices: [
            VertPos::new(center.x - half_sizes.x, center.y + half_sizes.y, center.z - half_sizes.z),
            VertPos::new(center.x - half_sizes.x, center.y + half_sizes.y, center.z + half_sizes.z),
            VertPos::new(center.x - half_sizes.x, center.y - half_sizes.y, center.z + half_sizes.z),
            VertPos::new(center.x - half_sizes.x, center.y - half_sizes.y, center.z - half_sizes.z),
            ],
          color,
          direction: QuadDirection::Left
        }
      }
      QuadDirection::Right => {
        Quad{
          vertices: [
            VertPos::new(center.x + half_sizes.x, center.y + half_sizes.y, center.z + half_sizes.z),
            VertPos::new(center.x + half_sizes.x, center.y + half_sizes.y, center.z - half_sizes.z),
            VertPos::new(center.x + half_sizes.x, center.y - half_sizes.y, center.z - half_sizes.z),
            VertPos::new(center.x + half_sizes.x, center.y - half_sizes.y, center.z + half_sizes.z),
            ],
          color,
          direction: QuadDirection::Right
        }
      }
    }
  }
}
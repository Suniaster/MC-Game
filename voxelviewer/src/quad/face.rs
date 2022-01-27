
use super::{VertPos, VectorDir};
use super::Quad;
pub enum QuadDirection{
  Front, Back, Left, Right, Up, Down
}


impl QuadDirection{
  pub fn from_dir(dir: &QuadDirection, center: &VertPos, half_sizes: &VectorDir)->Quad{
    let color = [0.1, 0.2, 0.4];
    match dir{
      QuadDirection::Front => {
        Quad{
          vertices: [
            VertPos::new(center.x + half_sizes.x, center.y + half_sizes.y, center.z - half_sizes.z),
            VertPos::new(center.x - half_sizes.x, center.y + half_sizes.y, center.z - half_sizes.z),
            VertPos::new(center.x - half_sizes.x, center.y - half_sizes.y, center.z - half_sizes.z),
            VertPos::new(center.x + half_sizes.x, center.y - half_sizes.y, center.z - half_sizes.z),
            ],
          color
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
          color
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
          color
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
          color
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
          color
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
          color
        }
      }
    }
  }
}
use nalgebra::{Point3, Vector3};

use super::CubeFace;

#[derive(PartialEq, Eq)]
pub enum CubeFaceDirection{
  Front, Back, Left, Right, Up, Down
}

impl CubeFaceDirection{
  pub fn cube_face_from_dir(dir: &CubeFaceDirection, half_sizes: &Vector3<f32>, color: [f32; 3])->CubeFace{
    let center = Point3::<f32>::origin();
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
        CubeFace{
          vertices: [
            Point3::new(center.x + half_sizes.x, center.y + half_sizes.y, center.z - half_sizes.z),
            Point3::new(center.x - half_sizes.x, center.y + half_sizes.y, center.z - half_sizes.z),
            Point3::new(center.x - half_sizes.x, center.y - half_sizes.y, center.z - half_sizes.z),
            Point3::new(center.x + half_sizes.x, center.y - half_sizes.y, center.z - half_sizes.z),
            ],
          color,
          direction: CubeFaceDirection::Front
        }
      }
      CubeFaceDirection::Back => {
        CubeFace{
          vertices: [
            Point3::new(center.x - half_sizes.x, center.y + half_sizes.y, center.z + half_sizes.z),
            Point3::new(center.x + half_sizes.x, center.y + half_sizes.y, center.z + half_sizes.z),
            Point3::new(center.x + half_sizes.x, center.y - half_sizes.y, center.z + half_sizes.z),
            Point3::new(center.x - half_sizes.x, center.y - half_sizes.y, center.z + half_sizes.z),
            ],
          color,
          direction: CubeFaceDirection::Back
        }
      }
      CubeFaceDirection::Up => {
        CubeFace{
          vertices: [
            Point3::new(center.x + half_sizes.x, center.y + half_sizes.y, center.z + half_sizes.z),
            Point3::new(center.x - half_sizes.x, center.y + half_sizes.y, center.z + half_sizes.z),
            Point3::new(center.x - half_sizes.x, center.y + half_sizes.y, center.z - half_sizes.z),
            Point3::new(center.x + half_sizes.x, center.y + half_sizes.y, center.z - half_sizes.z),
            ],
          color,
          direction: CubeFaceDirection::Up
        }
      }
      CubeFaceDirection::Down => {
        CubeFace{
          vertices: [
            Point3::new(center.x + half_sizes.x, center.y - half_sizes.y, center.z - half_sizes.z),
            Point3::new(center.x - half_sizes.x, center.y - half_sizes.y, center.z - half_sizes.z),
            Point3::new(center.x - half_sizes.x, center.y - half_sizes.y, center.z + half_sizes.z),
            Point3::new(center.x + half_sizes.x, center.y - half_sizes.y, center.z + half_sizes.z),
            ],
          color,
          direction: CubeFaceDirection::Down
        }
      }
      CubeFaceDirection::Left => {
        CubeFace{
          vertices: [
            Point3::new(center.x - half_sizes.x, center.y + half_sizes.y, center.z - half_sizes.z),
            Point3::new(center.x - half_sizes.x, center.y + half_sizes.y, center.z + half_sizes.z),
            Point3::new(center.x - half_sizes.x, center.y - half_sizes.y, center.z + half_sizes.z),
            Point3::new(center.x - half_sizes.x, center.y - half_sizes.y, center.z - half_sizes.z),
            ],
          color,
          direction: CubeFaceDirection::Left
        }
      }
      CubeFaceDirection::Right => {
        CubeFace{
          vertices: [
            Point3::new(center.x + half_sizes.x, center.y + half_sizes.y, center.z + half_sizes.z),
            Point3::new(center.x + half_sizes.x, center.y + half_sizes.y, center.z - half_sizes.z),
            Point3::new(center.x + half_sizes.x, center.y - half_sizes.y, center.z - half_sizes.z),
            Point3::new(center.x + half_sizes.x, center.y - half_sizes.y, center.z + half_sizes.z),
            ],
          color,
          direction: CubeFaceDirection::Right
        }
      }
    }
  }
}
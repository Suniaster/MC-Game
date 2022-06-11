pub mod cube_face_direction;

use cube_face_direction::CubeFaceDirection;
use nalgebra::{Point3, Vector3};

pub struct CubeFace{
    pub vertices: [Point3<f32>; 4],
    pub color: [f32; 3],
    pub direction: CubeFaceDirection
}

impl CubeFace{
    pub fn move_vertices(&mut self, disloc: &Vector3<f32>){
        for i in 0..4{
            self.vertices[i] += disloc;
        }
    }
}
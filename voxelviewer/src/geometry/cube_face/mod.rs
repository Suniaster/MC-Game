pub mod cube_face_direction;

use cube_face_direction::CubeFaceDirection;
use nalgebra::{Point3, Vector3};

use crate::draw::mesh::vertex::Vertex;

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

    pub fn to_static_vertex_list(&self) -> Vec<Vertex>{
        let mut result:Vec<Vertex> = vec![];
        let v1 = self.vertices[0];
        let v2 = self.vertices[1];
        let v3 = self.vertices[2];
        
        let quad_dir = (v2 - v3).cross(&(v1 - v3));
        let quad_normal:[f32; 3] = quad_dir.normalize().into();

        let vertices = [
            self.vertices[0],
            self.vertices[1],
            self.vertices[2],
            self.vertices[3],
        ];

        for v in vertices{
            result.push(Vertex{
                color_diffuse: self.color,
                position: v.into(),
                normal: quad_normal
            });
        }

        return result;
    } 

    pub fn to_outline_vertex_list(&self) -> Vec<Vertex>{
        let mut result:Vec<Vertex> = vec![];
        let v1 = self.vertices[0];
        let v2 = self.vertices[1];
        let v3 = self.vertices[2];
        
        let quad_dir = (v2 - v3).cross(&(v1 - v3));
        let quad_normal:[f32; 3] = quad_dir.normalize().into();
         
        let vertices = [
            self.vertices[0], 
            self.vertices[1],
            
            self.vertices[1],
            self.vertices[2],
            
            self.vertices[2],
            self.vertices[3],

            self.vertices[3],
            self.vertices[0],
        ];

        for v in vertices{
            result.push(Vertex{
                color_diffuse: [0., 0., 0.],
                position: v.into(),
                normal: quad_normal
            });
        }
        return result;
    }
}
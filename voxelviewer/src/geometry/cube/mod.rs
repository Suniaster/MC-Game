use nalgebra::Vector3;
use nalgebra::Point3;

use super::super::vertex::{
  static_vertex::StaticVertex, StaticVertexMesh
};
use super::cube_face::{CubeFace, cube_face_direction::*};

pub struct Cuboid{
    pub center_position: Point3<f32>,
    _half_sizes: Vector3<f32>,
    faces: Vec<CubeFace>
}

impl Cuboid{
    pub fn new(center_position: Point3<f32>, half_sizes: Vector3<f32>, color:[f32;3])-> Self{
        let faces_dirs = vec![
            CubeFaceDirection::Front,
            CubeFaceDirection::Back,
            CubeFaceDirection::Up,
            CubeFaceDirection::Down,
            CubeFaceDirection::Left,
            CubeFaceDirection::Right,
        ];
    
        let faces:Vec<CubeFace> = faces_dirs.iter().map( |f| {
            CubeFaceDirection::cube_face_from_dir(f, &half_sizes, color)
        }).collect::<Vec<_>>();
        Self{
            faces, center_position, _half_sizes: half_sizes
        }
    }
    
    pub fn build_from_array(position:Point3<f32>, arr: &Vec<Cuboid>) -> StaticVertexMesh{
        let mut vertices = vec![];
        for hex in arr{
            for quad in &hex.faces {
                vertices.append(&mut quad.to_static_vertex_list());
            }
        }

        return StaticVertexMesh::new(
            vertices, 
            position.into()
        );
    }

    pub fn build_from_grid(position:Point3<f32>, grid: &Vec<Vec<Vec<Cuboid>>>) -> StaticVertexMesh{
        let mut vertices = vec![];
        for row in grid{
            for col in row{
                for hex in col{
                    for quad in &hex.faces {
                        vertices.append(&mut quad.to_static_vertex_list());
                    }
                }
            }
        }
        return StaticVertexMesh::new(
            vertices, 
            position.into()
        );
    }
    pub fn get_static_vertices(&self) -> Vec<StaticVertex>{
        let mut result = vec![];
        println!("{:?}", self.center_position);
        for quad in &self.faces {
            println!("{:?}", quad.vertices);
            result.append(&mut quad.to_static_vertex_list());
        }
        result
    }

    pub fn get_outline_vertices(&self) -> Vec<StaticVertex>{
        let mut result = vec![];
        for quad in &self.faces {
            result.append(&mut quad.to_outline_vertex_list());
        }
        result
    }

    pub fn remove_face(&mut self, dir: CubeFaceDirection){
        self.faces.retain(|quad| quad.direction != dir);
    }
}

impl Cuboid{
    pub fn build(&self) -> StaticVertexMesh{
        return StaticVertexMesh::new(
            self.get_static_vertices(), 
            self.center_position.into()
        );
    }
    pub fn build_outline(&self) -> StaticVertexMesh{
        return StaticVertexMesh::new(
            self.get_outline_vertices(), 
            self.center_position.into()
        );
    }
}
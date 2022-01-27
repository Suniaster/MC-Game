
use super::{VertPos, VectorDir, Quad};
use super::face::*;
use super::super::vertex::{static_vertex::StaticVertex};

pub struct HexagonMesh{
    pub center_position: VertPos,
    _half_sizes: VectorDir,
    faces: Vec<Quad>
}

impl HexagonMesh{
    pub fn new(center_position: VertPos, half_sizes: VectorDir)-> Self{
        let faces_dirs = vec![
            QuadDirection::Front,
            QuadDirection::Back,
            QuadDirection::Up,
            QuadDirection::Down,
            QuadDirection::Left,
            QuadDirection::Right,
        ];
    
        let faces:Vec<Quad> = faces_dirs.iter().map( |f| {
            QuadDirection::from_dir(f, &center_position, &half_sizes)
        }).collect::<Vec<_>>();
        Self{
            faces, center_position, _half_sizes: half_sizes
        }
    }

    pub fn get_static_vertices(&self) -> Vec<StaticVertex>{
        let mut result = vec![];
        for quad in &self.faces {
            result.append(&mut quad.to_static_vertex_list());
        }
        result
    }
}
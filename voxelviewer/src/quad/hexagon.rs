
use super::{VertPos, VectorDir, Quad};
use super::face::*;
use super::super::vertex::{static_vertex::StaticVertex, StaticVertexMesh, StaticVertexBuild};

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
    
    pub fn build_from_array(position:VertPos, arr: &Vec<HexagonMesh>) -> StaticVertexMesh{
        let mut vertices = vec![];
        for hex in arr{
            for quad in &hex.faces {
                vertices.append(&mut quad.to_static_vertex_list());
            }
        }

        StaticVertexMesh{
            vertices,
            position
        }
    }

    pub fn get_static_vertices(&self) -> Vec<StaticVertex>{
        let mut result = vec![];
        for quad in &self.faces {
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

    pub fn remove_face(&mut self, dir: QuadDirection){
        self.faces.retain(|quad| quad.direction != dir);
    }
}

pub struct HexagonMeshOutLine<'a>{
    pub hex: &'a HexagonMesh
}

impl StaticVertexBuild for HexagonMesh{
    fn build(&self) -> StaticVertexMesh{
        StaticVertexMesh{
            vertices: self.get_static_vertices(),
            position: self.center_position
        }
    }
}

impl StaticVertexBuild for HexagonMeshOutLine<'_>{
    fn build(&self) -> StaticVertexMesh{
        StaticVertexMesh{
            vertices: self.hex.get_outline_vertices(),
            position: self.hex.center_position
        }
    }
}
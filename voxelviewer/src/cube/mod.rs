use super::vertex::{
  static_vertex::StaticVertex, StaticVertexMesh
};
use super::quad::{Quad, cube_face_direction::*, VectorDir, VertPos};

pub struct Cuboid{
    pub center_position: VertPos,
    _half_sizes: VectorDir,
    faces: Vec<Quad>
}

impl Cuboid{
    pub fn new(center_position: VertPos, half_sizes: VectorDir, color:[f32;3])-> Self{
        let faces_dirs = vec![
            CubeFaceDirection::Front,
            CubeFaceDirection::Back,
            CubeFaceDirection::Up,
            CubeFaceDirection::Down,
            CubeFaceDirection::Left,
            CubeFaceDirection::Right,
        ];
    
        let faces:Vec<Quad> = faces_dirs.iter().map( |f| {
            CubeFaceDirection::quad_from_dir(f, &center_position, &half_sizes, color)
        }).collect::<Vec<_>>();
        Self{
            faces, center_position, _half_sizes: half_sizes
        }
    }
    
    pub fn build_from_array(position:VertPos, arr: &Vec<Cuboid>) -> StaticVertexMesh{
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

    pub fn _remove_face(&mut self, dir: CubeFaceDirection){
        self.faces.retain(|quad| quad.direction != dir);
    }
}

impl Cuboid{
    pub fn build(&self) -> StaticVertexMesh{
        StaticVertexMesh{
            vertices: self.get_static_vertices(),
            position: self.center_position
        }
    }
    pub fn build_outline(&self) -> StaticVertexMesh{
        StaticVertexMesh{
            vertices: self.get_outline_vertices(),
            position: self.center_position
        }
    }
}


pub fn _new_cube(half_size: f32) -> StaticVertexMesh {
  let cube = Cuboid::new(
    cgmath::Vector3::new(0.,0.,0.), 
    cgmath::Vector3::new(half_size,half_size,half_size),
    [0.1, 1.0, 0.1]
  );
  cube.build()
}


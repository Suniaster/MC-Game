use super::vertex::{
  static_vertex::StaticVertex, StaticVertexMesh, StaticVertexBuild
};
use super::quad::{Quad, quad_direction::*, VectorDir, VertPos};

pub struct Cuboid{
    pub center_position: VertPos,
    _half_sizes: VectorDir,
    faces: Vec<Quad>
}

impl Cuboid{
    pub fn new(center_position: VertPos, half_sizes: VectorDir, color:[f32;3])-> Self{
        let faces_dirs = vec![
            QuadDirection::Front,
            QuadDirection::Back,
            QuadDirection::Up,
            QuadDirection::Down,
            QuadDirection::Left,
            QuadDirection::Right,
        ];
    
        let faces:Vec<Quad> = faces_dirs.iter().map( |f| {
            QuadDirection::quad_from_dir(f, &center_position, &half_sizes, color)
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

    pub fn _remove_face(&mut self, dir: QuadDirection){
        self.faces.retain(|quad| quad.direction != dir);
    }
}

pub struct CuboidOutline<'a>{
    pub hex: &'a Cuboid
}

impl StaticVertexBuild for Cuboid{
    fn build(&self) -> StaticVertexMesh{
        StaticVertexMesh{
            vertices: self.get_static_vertices(),
            position: self.center_position
        }
    }
}

impl StaticVertexBuild for CuboidOutline<'_>{
    fn build(&self) -> StaticVertexMesh{
        StaticVertexMesh{
            vertices: self.hex.get_outline_vertices(),
            position: self.hex.center_position
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


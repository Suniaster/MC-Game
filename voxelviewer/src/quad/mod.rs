
use cgmath::Vector3;
use cgmath::InnerSpace;
use super::vertex::static_vertex::StaticVertex;

pub mod face;
pub mod hexagon;

type VertPos = Vector3<f32>;
type VectorDir = Vector3<f32>;

pub struct Quad{
    pub vertices: [VertPos; 4],
    pub color: [f32; 3],
    pub direction: face::QuadDirection
}

impl Quad{
    pub fn to_static_vertex_list(&self) -> Vec<StaticVertex>{
        let mut result:Vec<StaticVertex> = vec![];
        let v1 = self.vertices[0];
        let v2 = self.vertices[1];
        let v3 = self.vertices[2];
        
        let quad_dir = (v2 - v3).cross(v1 - v3);
        let quad_normal:[f32; 3] = quad_dir.normalize().into();

        let vertices = [
            self.vertices[2],
            self.vertices[1],
            self.vertices[0],

            self.vertices[3],
            self.vertices[2],
            self.vertices[0],
        ];

        for v in vertices{
            result.push(StaticVertex{
                color_diffuse: self.color,
                position: v.into(),
                normal: quad_normal
            });
        }

        return result;
    } 
}
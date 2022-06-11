use crate::{
    geometry::cube_face::CubeFace, 
    draw::mesh::vertex::Vertex
};


pub fn cube_face_to_vertex_list(cube_face: &CubeFace) -> Vec<Vertex> {
    let mut result: Vec<Vertex> = vec![];
    let v1 = cube_face.vertices[0];
    let v2 = cube_face.vertices[1];
    let v3 = cube_face.vertices[2];

    let quad_dir = (v2 - v3).cross(&(v1 - v3));
    let quad_normal: [f32; 3] = quad_dir.normalize().into();

    let vertices = [
        cube_face.vertices[0],
        cube_face.vertices[1],
        cube_face.vertices[2],
        cube_face.vertices[3],
    ];

    for v in vertices {
        result.push(Vertex {
            color_diffuse: cube_face.color,
            position: v.into(),
            normal: quad_normal,
        });
    }

    return result;
}
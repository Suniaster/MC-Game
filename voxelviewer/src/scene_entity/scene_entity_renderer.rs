use nalgebra::Point3;
use wgpu::util::DeviceExt;
use super::super::vertex::{StaticVertexMesh};


pub struct SceneEntityRenderer{
  pub vertex_buffer: wgpu::Buffer,
  pub index_buffer: wgpu::Buffer,
  pub num_indices: u32,
  pub num_vertices: u32
}


impl SceneEntityRenderer{
    pub fn new(device: &wgpu::Device, pos: Point3<f32>, mesh: &mut StaticVertexMesh)->SceneEntityRenderer{
        mesh.update_pos(pos);

        let vertex_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                contents: mesh.to_buffer()
            }
        );

        let indices = mesh.get_indices_for_square_mesh();
        let index_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                usage: wgpu::BufferUsages::INDEX,
                contents: bytemuck::cast_slice(&indices)
            }
        );
        SceneEntityRenderer{
            index_buffer,
            num_indices: indices.len() as u32,
            num_vertices: mesh.vertices.len() as u32,
            vertex_buffer,
        }
    }
}



pub fn draw_entity<'a, 'b>(
    render_pas: &mut wgpu::RenderPass<'a>,
    entity: &'a SceneEntityRenderer,
    camera_bind_group: &'a wgpu::BindGroup
){
    render_pas.set_bind_group(0, camera_bind_group, &[]);
    render_pas.set_vertex_buffer(0, entity.vertex_buffer.slice(..));

    render_pas.set_index_buffer(entity.index_buffer.slice(..), wgpu::IndexFormat::Uint32); // 1.
    render_pas.draw_indexed(0..entity.num_indices, 0, 0..1); // 2.
}

use wgpu::util::DeviceExt;
use super::vertex::{StaticVertexMesh};

pub struct SceneEntity{
  pub vertex_buffer: wgpu::Buffer,
  pub index_buffer: wgpu::Buffer,
  pub num_indices: u32,
  pub num_vertices: u32,
  pub instance: StaticVertexMesh,
  pub id: u32
}

static mut INSTANCE_ID: u32 = 0;

unsafe fn gen_instance_id() -> u32{
    INSTANCE_ID += 1;
    INSTANCE_ID
}

impl SceneEntity{
    pub fn new(device: &wgpu::Device, pos: cgmath::Vector3<f32>, mesh: StaticVertexMesh)->SceneEntity{
        // let mut instance = mesh.build();
        let mut mesh = mesh;
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

        let id: u32;
        unsafe{
            id = gen_instance_id();
        }
        SceneEntity{
            id, 
            index_buffer,
            num_indices: indices.len() as u32,
            num_vertices: mesh.vertices.len() as u32,
            instance: mesh,
            vertex_buffer,
        }
    }

    pub fn update_pos(&mut self, queue: &wgpu::Queue, new_pos: cgmath::Vector3<f32>){
        if self.instance.update_pos(new_pos) {
            queue.write_buffer(
                &self.vertex_buffer, 
                0, 
                self.instance.to_buffer()
            );
        }
    }
}

pub trait DrawModel<'a> {
    fn draw_entity(
        &mut self,
        entity: &'a SceneEntity,
        camera_bind_group: &'a wgpu::BindGroup
    );
}

impl<'a, 'b> DrawModel<'b> for wgpu::RenderPass<'a>
where
    'b: 'a,
{
    fn draw_entity(
        &mut self,
        entity: &'a SceneEntity,
        camera_bind_group: &'a wgpu::BindGroup
    ){
        self.set_bind_group(0, camera_bind_group, &[]);
        self.set_vertex_buffer(0, entity.vertex_buffer.slice(..));

        self.set_index_buffer(entity.index_buffer.slice(..), wgpu::IndexFormat::Uint32); // 1.
        self.draw_indexed(0..entity.num_indices, 0, 0..1); // 2.
    }
}
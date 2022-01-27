use wgpu::util::DeviceExt;
use super::vertex::{StaticVertexMesh, StaticVertexBuild};
use super::cube::cube_mesh::{new_cube};
use super::grid;

pub struct SceneEntity{
  vertex_buffer: wgpu::Buffer,
  num_vertices: u32,
  instance: StaticVertexMesh,
  pub id: u32
}

static mut INSTANCE_ID: u32 = 0;

unsafe fn gen_instance_id() -> u32{
    INSTANCE_ID += 1;
    INSTANCE_ID
}

impl SceneEntity{
    pub fn new<T: StaticVertexBuild>(device: &wgpu::Device, pos: cgmath::Vector3<f32>, mesh: &T)->SceneEntity{
        let mut instance = mesh.build();
        instance.update_pos(pos);

        let vertex_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                contents: instance.to_buffer()
            }
        );

        let id: u32;
        unsafe{
            id = gen_instance_id();
        }
        SceneEntity{
            id, 
            num_vertices: instance.vertices.len() as u32,
            instance,
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
        camera_bind_group: &'a wgpu::BindGroup,
        light_bind_group: &'a wgpu::BindGroup,
    );
}

impl<'a, 'b> DrawModel<'b> for wgpu::RenderPass<'a>
where
    'b: 'a,
{
    fn draw_entity(
        &mut self,
        entity: &'a SceneEntity,
        camera_bind_group: &'a wgpu::BindGroup,
        light_bind_group: &'a wgpu::BindGroup,
    ){
        self.set_bind_group(0, camera_bind_group, &[]);
        self.set_bind_group(1, light_bind_group, &[]);
        
        self.set_vertex_buffer(0, entity.vertex_buffer.slice(..));

        self.draw(0..entity.num_vertices, 0..1);
    }
}
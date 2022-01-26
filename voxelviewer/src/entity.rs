use crate::voxel;
use wgpu::util::DeviceExt;

pub struct SceneEntity{
  instance_buffer: wgpu::Buffer,
  vertex_buffer: wgpu::Buffer,
  num_vertices: u32,
  instance: voxel::Instance,
  pub id: u32
}


impl SceneEntity{
    pub fn new(device: &wgpu::Device, pos: cgmath::Vector3<f32>)->SceneEntity{
        let instance = voxel::Instance::new(
            pos,
            [0.1, 0.1, 0.3]
        );

        let instance_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                contents: bytemuck::cast_slice(&[instance.to_raw()]),
            }
        );
        let default_cube = voxel::DefaultQuad::new();
        let vertices = default_cube.get_complete_vertexes();

        let vertex_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                usage: wgpu::BufferUsages::VERTEX,
                contents: bytemuck::cast_slice(&vertices),
            }
        );

        SceneEntity{
            id: instance.idx,
            instance_buffer, 
            instance,
            vertex_buffer,
            num_vertices: vertices.len() as u32,
        }
    }

    pub fn update_pos(&mut self, queue: &wgpu::Queue, new_pos: cgmath::Vector3<f32>){
        self.instance.position = new_pos;
        queue.write_buffer(
            &self.instance_buffer, 
            0, 
            bytemuck::cast_slice(&[self.instance.to_raw()])
        );
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
        self.set_vertex_buffer(1, entity.instance_buffer.slice(..));
        
        self.draw(0..entity.num_vertices, 0..1);
    }
}
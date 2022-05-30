pub mod scene_entity_renderer;

use super::vertex::StaticVertexMesh;
use scene_entity_renderer::SceneEntityRenderer;
use nalgebra::Point3;


static mut INSTANCE_ID: u32 = 0;

unsafe fn gen_instance_id() -> u32{
    INSTANCE_ID += 1;
    INSTANCE_ID
}


pub struct SceneEntity{
    pub renderer: SceneEntityRenderer,
    pub mesh: StaticVertexMesh,
    pub id: u32
}

impl SceneEntity{
    pub fn new(device: &wgpu::Device, mesh: StaticVertexMesh) -> SceneEntity{
        let mut mesh = mesh;
        let renderer = SceneEntityRenderer::new(device, Point3::origin(), &mut mesh);
        let id: u32;
        unsafe{
            id = gen_instance_id();
        }
        SceneEntity{
            id,
            renderer,
            mesh
        }
    }

    pub fn update_pos(&mut self, queue: &wgpu::Queue, new_pos: Point3<f32>){
        if self.mesh.update_pos(new_pos) {
            queue.write_buffer(
                &self.renderer.vertex_buffer, 
                0, 
                self.mesh.to_buffer()
            );
        }
    }
}


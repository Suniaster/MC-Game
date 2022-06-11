use std::{sync::{Arc, Mutex}, iter};

use nalgebra::Point3;
use specs::prelude::*;
use crate::geometry::{grid::{self, GridMatrix}};

pub mod boundingbox;

use crate::{
    ScreenView, 
    scene_entity::scene_entity_renderer::SceneEntityRenderer, 
    vertex::StaticVertexMesh,
};

/************* VIEW COMPONENT ***************/
pub struct PositionComponent(Point3<f32>);
impl Component for PositionComponent {type Storage = VecStorage<Self>;}
impl PositionComponent {pub fn new(position: Point3<f32>) -> Self {Self(position)}}
pub struct MeshRenderer{
    pub renderer: Option<SceneEntityRenderer>,
    pub mesh: StaticVertexMesh
}

impl Component for MeshRenderer {
    type Storage = VecStorage<Self>;
}

impl MeshRenderer {
    pub fn new(device: &wgpu::Device, mesh: StaticVertexMesh) -> MeshRenderer {
        let mut mesh = mesh;
        let renderer = SceneEntityRenderer::new(device, &mut mesh);
        MeshRenderer{
            renderer: Some(renderer),
            mesh
        }
    }

    pub fn create_without_renderer(mesh: StaticVertexMesh) -> Self{
        MeshRenderer{
            renderer: None,
            mesh
        }
    }

    pub fn from_grid(cube_size: f32, grid: GridMatrix) -> Self {
        let grid_mesh = grid::Grid::create_with(
            cube_size
        );

        let mesh = grid_mesh.build_from(&grid);
        MeshRenderer::create_without_renderer(
            mesh
        )
    }

    pub fn update_origin(&mut self, queue: &wgpu::Queue, new_pos: Point3<f32>){
        if self.mesh.update_origin(new_pos.into()) {
            if let Some(renderer) = &self.renderer {
                queue.write_buffer(
                    &renderer.instance_buffer, 
                    0, 
                    self.mesh.to_instance_buffer()
                );
            }
        }
    }

    pub fn update_renderer(&mut self, device: &wgpu::Device) {
        let renderer = SceneEntityRenderer::new(device, &mut self.mesh);
        self.renderer = Some(renderer);
    }


}


/*************** VIEW SYSTEM *******************/
pub struct ViewSystem{
    state: Arc<Mutex<ScreenView>>,
}
impl ViewSystem {
    pub fn new(state: Arc<Mutex<ScreenView>>) -> Self {
        Self {
            state,
        }
    }
}

impl <'a> System <'a> for ViewSystem {
    type SystemData = 
        ReadStorage<'a, MeshRenderer>
    ;

    fn setup(&mut self, world: &mut specs::World) {
        Self::SystemData::setup(world);
        world.register::<MeshRenderer>();
        world.register::<PositionComponent>();
    }

    fn run(&mut self, meshes: Self::SystemData) {
        let mut view = self.state.lock().unwrap();
        let state = &mut view.state;
        let output = state.surface.get_current_texture().unwrap();

        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = state
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: true,
                    },
                }],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &state.depth_texture.view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: true,
                    }),
                    stencil_ops: None,
                }),
            });

            render_pass.set_pipeline(&state.static_cube_pipeline);
            for mesh in (&meshes).join() {
                let renderer = &mesh.renderer;
                if let Some(renderer) = renderer {
                    render_pass.set_bind_group(0, &state.camera_bind_group, &[]);
                    render_pass.set_vertex_buffer(0, renderer.vertex_buffer.slice(..));
    
                    render_pass.set_vertex_buffer(1, renderer.instance_buffer.slice(..));
                    
                    render_pass.set_index_buffer(renderer.index_buffer.slice(..), wgpu::IndexFormat::Uint32); // 1.
                    render_pass.draw_indexed(0..renderer.num_indices, 0, 0..1); // 2.
                }
            }
        }
        
  

        state.queue.submit(iter::once(encoder.finish()));
        output.present();

    }
}


/*************** VIEW SYSTEM *******************/
pub struct UpdateViewMeshesSystem{
    state: Arc<Mutex<ScreenView>>
}

impl UpdateViewMeshesSystem {
    pub fn new(state: Arc<Mutex<ScreenView>>) -> Self {
        Self {
            state,
        }
    }
}
impl <'a> System <'a> for UpdateViewMeshesSystem {
    type SystemData = (
        ReadStorage<'a, PositionComponent>,
        WriteStorage<'a, MeshRenderer>,
    );

    fn run(&mut self, (positions, mut meshes): Self::SystemData) {
        let mut view = self.state.lock().unwrap();
        let state = &mut view.state;

        for (pos, mesh) in (&positions, &mut meshes).join() {
            if mesh.renderer.is_none(){
                mesh.mesh.update_origin(pos.0.into());
                mesh.update_renderer(&state.device);
            }
            else{
                mesh.update_origin(&state.queue, pos.0);
            }
        }
    }
}
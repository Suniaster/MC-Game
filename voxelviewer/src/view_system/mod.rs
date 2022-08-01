use std::{sync::{Arc, Mutex}, iter};

use plugins::PluginSytem;
use specs::prelude::*;
use crate::{screen_text::ScreenText, scene::State};

pub mod boundingbox;
pub mod camera_system;
pub mod components;
pub mod resources;

use crate::{
    ScreenView
};

use self::components::{PositionComponent, MeshRendererComponent};

/*************** VIEW SYSTEM *******************/
pub struct ViewSystem;

impl PluginSytem<'_> for ViewSystem {
    fn name(&self) -> &'static str {
        "voxel_viewer_view_system"
    }
}

impl <'a> System <'a> for ViewSystem {
    type SystemData = (
        WriteExpect<'a, Mutex<State>>,
        ReadStorage<'a, MeshRendererComponent>,
        Read<'a, Vec<ScreenText>>
    );

    fn run(&mut self, (mut state_mutex, meshes, texts): Self::SystemData) {
        let mut state = state_mutex.lock().unwrap();
        let output = state.surface.get_current_texture().unwrap();

        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = state
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.1,
                        g: 0.7,
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
                render_pass.set_bind_group(1, &&state.depth_bind_group, &[]);
                
                render_pass.set_vertex_buffer(0, renderer.vertex_buffer.slice(..));

                render_pass.set_vertex_buffer(1, renderer.instance_buffer.slice(..));
                
                render_pass.set_index_buffer(renderer.index_buffer.slice(..), wgpu::IndexFormat::Uint32); // 1.
                render_pass.draw_indexed(0..renderer.num_indices, 0, 0..1); // 2.
            }
        }
        
        
        // for text in texts.iter() {
        //     text.draw(
        //         &mut state.glyph_brush, 
        //         state.size.width as f32, 
        //         state.size.height as f32
        //     )
        // }

        // state.glyph_brush
        //     .draw_queued(
        //         &state.device,
        //         &mut state.staging_belt,
        //         &mut encoder,
        //         &view,
        //         state.size.width,
        //         state.size.height,
        //     )
        //     .expect("Draw queued");
        
        drop(render_pass);
        state.staging_belt.finish();
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
        WriteStorage<'a, MeshRendererComponent>,
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
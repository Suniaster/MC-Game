


use std::iter;

use wgpu::util::DeviceExt;
use wgpu_glyph::GlyphBrush;
use winit::{
    event::*,
    // event_loop::{ControlFlow, EventLoop},
    window::Window,
};

use crate::texture;
use crate::camera;
use std::collections::HashMap;

use super::screen_text::ScreenText;
use crate::scene_entity::{SceneEntity};
use crate::scene_entity::scene_entity_renderer::draw_entity;
use wgpu_glyph::{ab_glyph, GlyphBrushBuilder};

use nalgebra::Matrix4;

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct CameraUniform {
    view_proj: [[f32; 4]; 4],
    view_position: [f32; 4],
}

impl CameraUniform {
    fn new() -> Self {
        Self {
            view_position: [0.0; 4],
            view_proj: Matrix4::identity().into(),
        }
    }

    fn update_view_proj(&mut self, camera: &camera::Camera, projection: &camera::Projection) {
        self.view_position = camera.position.to_homogeneous().into();
        self.view_proj = (projection.calc_matrix() * camera.calc_matrix()).into();
    }
}

pub struct State {
    pub surface: wgpu::Surface,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,

    pub depth_texture: texture::Texture,

    pub static_cube_pipeline: wgpu::RenderPipeline,
    pub static_lines_pipeline: wgpu::RenderPipeline,

    //Glyph
    glyph_brush: GlyphBrush<()>,
    staging_belt:  wgpu::util::StagingBelt,
    pub screen_texts: Vec<ScreenText>,

    // Camera
    pub camera: camera::Camera,
    pub camera_controller: camera::CameraController,
    camera_uniform: CameraUniform,
    camera_buffer: wgpu::Buffer,
    camera_bind_group: wgpu::BindGroup,
    projection: camera::Projection,

    // Instances
    pub entities: HashMap<u32, SceneEntity>,
    pub entities_outlines: HashMap<u32, SceneEntity>,

    // Input: bool
    mouse_pressed: bool,
}

impl State {
    pub async fn new(window: &Window) -> Self {
        let size = window.inner_size();

        // The instance is a handle to our GPU
        // Backends::all => Vulkan + Metal + DX12 + Browser WebGPU
        let instance = wgpu::Instance::new(wgpu::Backends::all());
        let surface = unsafe { instance.create_surface(window) };
        let adapter = instance.request_adapter(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            },
        ).await.unwrap();

        let (device, queue) = adapter.request_device(
            &wgpu::DeviceDescriptor {
                features: wgpu::Features::empty(),
                limits: wgpu::Limits::default(),
                label: None,
            },
            None, // Trace path
        ).await.unwrap();

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_preferred_format(&adapter).unwrap(),
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Immediate,
        };
        surface.configure(&device, &config);
        let depth_texture =
            texture::Texture::create_depth_texture(&device, &config, "depth_texture");

        // ********** CAMERA
        let camera = camera::Camera::new(
            nalgebra::Point3::new(0.0, 5.0, 10.0), 
            nalgebra::UnitComplex::new(-1.), 
            nalgebra::UnitComplex::new(-0.342)
        );
        let projection = camera::Projection::new(
            config.width, 
            config.height, 
            nalgebra::UnitComplex::new(std::f32::consts::FRAC_PI_4), 
            0.1, 
            1000.0
        );
        let camera_controller = camera::CameraController::new(4.0, 0.4);

        // in new() after creating `camera`

        let mut camera_uniform = CameraUniform::new();
        camera_uniform.update_view_proj(&camera, &projection);

        let camera_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Camera Buffer"),
                contents: bytemuck::cast_slice(&[camera_uniform]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            }
        );

        let camera_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }
            ],
            label: Some("camera_bind_group_layout"),
        });
        let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &camera_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: camera_buffer.as_entire_binding(),
                }
            ],
            label: Some("camera_bind_group"),
        });


        let static_cube_pipeline = super::pipelines::static_vertex_pipeline::create_cube_render_pipeline(
            &device, 
            &[
                &camera_bind_group_layout,
            ], 
            &config,
            wgpu::PrimitiveTopology::TriangleList
        );

        let static_lines_pipeline =  super::pipelines::static_vertex_pipeline::create_cube_render_pipeline(
            &device, 
            &[
                &camera_bind_group_layout,
            ], 
            &config,
            wgpu::PrimitiveTopology::LineList
        );

        // Prepare glyph_brush
        let render_format = wgpu::TextureFormat::Bgra8UnormSrgb;
        let inconsolata = ab_glyph::FontArc::try_from_slice(include_bytes!(
            "Inconsolata-Regular.ttf"
        )).expect("Failed to load font");
    
        let glyph_brush = GlyphBrushBuilder::using_font(inconsolata)
            .build(&device, render_format);
        let staging_belt = wgpu::util::StagingBelt::new(1024);
        Self {
            size,
            surface,
            device,
            queue,
            config,
            depth_texture,
            camera,
            projection,
            camera_uniform,
            camera_buffer,
            camera_bind_group,    
            camera_controller,
            entities: HashMap::new(),
            entities_outlines: HashMap::new(),
            
            static_cube_pipeline,
            static_lines_pipeline,

            mouse_pressed: false,

            glyph_brush, 
            staging_belt,
            screen_texts: vec![]
        }
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        // UPDATED!
        if new_size.width > 0 && new_size.height > 0 {
            self.projection.resize(new_size.width, new_size.height);
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
            self.depth_texture =
                texture::Texture::create_depth_texture(&self.device, &self.config, "depth_texture");
        }
    }

    // UPDATED!
    pub fn input(&mut self, event: &DeviceEvent) -> bool {
        match event {
            DeviceEvent::Key(
                KeyboardInput {
                    virtual_keycode: Some(key),
                    state,
                    ..
                }
            ) => self.camera_controller.process_keyboard(*key, *state),
            DeviceEvent::MouseWheel { delta, .. } => {
                self.camera_controller.process_scroll(delta);
                true
            }
            DeviceEvent::Button {
                button: 1, // Left Mouse Button
                state,
            } => {
                self.mouse_pressed = *state == ElementState::Pressed;
                true
            }
            DeviceEvent::MouseMotion { delta } => {
                if self.mouse_pressed {
                    self.camera_controller.process_mouse(delta.0, delta.1);
                }
                true
            }
            _ => false,
        }
    }

    pub fn update(&mut self, dt: std::time::Duration) {
        self.camera_controller.update_camera(&mut self.camera, dt);
        self.camera_uniform.update_view_proj(&self.camera, &self.projection);

        self.queue.write_buffer(&self.camera_buffer, 0, bytemuck::cast_slice(&[self.camera_uniform]));
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
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
                    view: &self.depth_texture.view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: true,
                    }),
                    stencil_ops: None,
                }),
            });

            render_pass.set_pipeline(&self.static_cube_pipeline);
            for (_, ent) in &self.entities{
                draw_entity(
                    &mut render_pass,
                    &ent.renderer,
                    &self.camera_bind_group
                );
            }

            render_pass.set_pipeline(&self.static_lines_pipeline);
            for(_, ent) in &self.entities_outlines{
                render_pass.set_bind_group(0, &self.camera_bind_group, &[]);
                render_pass.set_vertex_buffer(0, ent.renderer.vertex_buffer.slice(..));
                render_pass.set_vertex_buffer(1, ent.renderer.instance_buffer.slice(..));
                render_pass.draw(0..ent.renderer.num_vertices, 0..1);
            }
        }
        
        for text in self.screen_texts.iter(){
            text.draw(
                &mut self.glyph_brush, 
                self.size.width as f32, 
                self.size.height as f32
            )
        }

        self.glyph_brush
            .draw_queued(
                &self.device,
                &mut self.staging_belt,
                &mut encoder,
                &view,
                self.size.width,
                self.size.height,
            )
            .expect("Draw queued");
        self.staging_belt.finish();
        self.queue.submit(iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}

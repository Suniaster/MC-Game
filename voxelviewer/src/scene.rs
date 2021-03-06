use wgpu::util::DeviceExt;
use wgpu_glyph::GlyphBrush;
use winit::{
    event::*,
    // event_loop::{ControlFlow, EventLoop},
    window::Window,
};

use crate::camera::Camera;
use crate::texture;
use crate::camera;
use crate::draw::pipelines::create_cube_render_pipeline;

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

    pub static_cube_pipeline: wgpu::RenderPipeline,

    //Glyph
    pub glyph_brush: GlyphBrush<()>,
    pub staging_belt:  wgpu::util::StagingBelt,

    // Camera
    pub camera: camera::Camera,
    pub camera_controller: camera::CameraController,
    camera_uniform: CameraUniform,
    camera_buffer: wgpu::Buffer,
    pub camera_bind_group: wgpu::BindGroup,
    projection: camera::Projection,

    // Textures for better drawing
    pub depth_texture: texture::Texture,
    pub depth_bind_group: wgpu::BindGroup,

    // Input: bool
    mouse_pressed: bool,
}

impl State {
    pub async fn new(window: &Window) -> Self {

        /******** Creating Configuration and control Variables ***********/
        let size = window.inner_size();
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
            present_mode: wgpu::PresentMode::Fifo,
        };
        surface.configure(&device, &config);
        let depth_texture =
            texture::Texture::create_depth_texture(&device, &config, "depth_texture");

        /************** CAMERA VARIABLES *************/
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

        let mut camera_uniform = CameraUniform::new();
        camera_uniform.update_view_proj(&camera, &projection);
        let camera_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Camera Buffer"),
                contents: bytemuck::cast_slice(&[camera_uniform]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            }
        );

        let camera_bind_group_layout = Camera::create_bind_gropu_layout(&device);
        let camera_bind_group = Camera::create_bind_group(&device, &camera_bind_group_layout, &camera_buffer);

        /*********** TEXTURES **************/
        let depth_bind_group_layout = texture::Texture::create_depth_texture_bindgroup_layouy(&device);
        
        // let t_bytes = include_bytes!("./img_lights.jpg");
        // let t_texture = texture::Texture::from_bytes(&device, &queue, t_bytes, "texture");
        // let depth_bind_group = t_texture.create_bind_group(&device, &depth_bind_group_layout);

        let depth_bind_group = depth_texture.create_bind_group(&device, &depth_bind_group_layout);

        /*********** PIPELINES **************/

        let static_cube_pipeline = create_cube_render_pipeline(
            &device, 
            &[
                &camera_bind_group_layout,
                &depth_bind_group_layout,
            ], 
            &config,
            wgpu::PrimitiveTopology::TriangleList
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
            
            static_cube_pipeline,

            depth_bind_group,

            mouse_pressed: false,

            glyph_brush, 
            staging_belt,
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
}

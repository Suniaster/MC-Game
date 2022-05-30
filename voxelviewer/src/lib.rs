use nalgebra::{Point3, Vector3};
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
};

mod texture; 
mod camera;
mod scene;
mod geometry;
mod pipelines;
mod screen_text;
mod scene_entity;
mod vertex;
use scene::*;
use geometry::*;
pub struct ViewActions{
    pub state: State
}

pub struct ViewObjectInfo{
    pub position: Point3<f32>,
    pub color: [f32; 3],
    pub size: [f32; 3],
    pub id: u32
}

impl ViewActions{

    pub fn create_grid(&mut self, position: [f32; 3], cube_size: f32, grid_mat: grid::GridMatrix) -> ViewObjectInfo{
        let mesh = grid::Grid::create_from(
            position, 
            cube_size,
            grid_mat
        );

        let new_ent = scene_entity::SceneEntity::new(
            &self.state.device,
            mesh.build()
        );

        let id = new_ent.id;
        self.state.entities.insert(id, new_ent);

        return ViewObjectInfo{
            position: mesh.position,
            color: [0., 0., 0.],
            size: [0., 0., 0.],
            id
        };
    }

    pub fn create_cube(&mut self, obj: ViewObjectInfo)-> ViewObjectInfo{
        let mesh = cube::Cuboid::new(
            obj.position,
            Vector3::from(obj.size)/2., 
            obj.color
        );

        let new_ent = scene_entity::SceneEntity::new(
            &self.state.device, 
            mesh.build()
        );

        let id = new_ent.id;
        let mut obj = obj;
        self.state.entities.insert(id, new_ent);

        obj.id = id;
        return obj;
    }

    pub fn update_cube(&mut self, obj: &ViewObjectInfo){
        let entity = self.state.entities.get_mut(&obj.id).unwrap();
        entity.update_pos(&self.state.queue, obj.position);
    }

    pub fn set_camera_pos(&mut self, pos: nalgebra::Point3<f32>){
        self.state.camera.position = pos;
    }

    pub fn set_camera_config(&mut self, speed: Option<f32>, sens: Option<f32>){
        if let Some(value) = speed {
            self.state.camera_controller.speed = value;
        }
        if let Some(value) = sens {
            self.state.camera_controller.sensitivity = value;
        }
    }

    pub fn update_text(&mut self, id: usize, text: String){
        self.state.screen_texts[id as usize].text = text;
    }

    pub fn create_text(&mut self)-> usize{
        let new_text = screen_text::ScreenText::new(
            String::from(""), 
            10., 
            20. * self.state.screen_texts.len() as f32, 
        [0., 0., 0., 1.]);
        self.state.screen_texts.push(new_text);
        return self.state.screen_texts.len()-1;
    }

    pub fn get_vertex_count(&self) -> u32{
        let mut count = 0;
        for (_, entity) in self.state.entities.iter(){
            count += entity.mesh.vertices.len() as u32;
        }
        return count;
    }
}

pub trait ViewController{
    fn on_update(&mut self, a:&mut ViewActions, b:std::time::Duration) -> ();
    fn on_keybord_input(&mut self, a: &mut ViewActions, b:VirtualKeyCode, c:ElementState) -> ();
    fn before_start(&mut self, a:&mut ViewActions) -> ();
}

pub fn main(controller: Box<dyn ViewController>){
    env_logger::init();
    let event_loop = EventLoop::new();
    let title = env!("CARGO_PKG_NAME");
    let window = winit::window::WindowBuilder::new()
        .with_title(title)
        .build(&event_loop)
        .unwrap();


    let mut actions = ViewActions{state: pollster::block_on(State::new(&window))};
    let mut last_render_time = std::time::Instant::now();
    let mut controller = controller;

    controller.before_start(&mut actions);
    event_loop.run(move |event, _, control_flow| {
        
        *control_flow = ControlFlow::Poll;
        match event {
            Event::MainEventsCleared => window.request_redraw(),
            Event::DeviceEvent {
                ref event,
                .. // We're not using device_id currently
            } => {
                actions.state.input(event);
                match event{
                    DeviceEvent::Key(
                        KeyboardInput {
                            virtual_keycode: Some(key),
                            state,
                            ..
                        }
                    ) => {
                        controller.on_keybord_input(&mut actions, *key, *state);
                    }
                    _ => {}
                }
            }
            // UPDATED!
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => {
                match event {
                    WindowEvent::CloseRequested
                    | WindowEvent::KeyboardInput {
                        input:
                        KeyboardInput {
                            state: ElementState::Pressed,
                            virtual_keycode: Some(VirtualKeyCode::Escape),
                            ..
                        },
                        ..
                    } => *control_flow = ControlFlow::Exit,
                    WindowEvent::Resized(physical_size) => {
                        actions.state.resize(*physical_size);
                    }
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        actions.state.resize(**new_inner_size);
                    }
                    _ => {}
                }
            }
            // UPDATED!
            Event::RedrawRequested(window_id) if window_id == window.id() => {
                let now = std::time::Instant::now();
                let dt = now - last_render_time;
                last_render_time = now;
                actions.state.update(dt);
                controller.on_update(&mut actions, dt);
                match actions.state.render() {
                    Ok(_) => {}
                    // Reconfigure the surface if lost
                    Err(wgpu::SurfaceError::Lost) => actions.state.resize(actions.state.size),
                    // The system is out of memory, we should probably quit
                    Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    // All other errors (Outdated, Timeout) should be resolved by the next frame
                    Err(e) => eprintln!("{:?}", e),
                }
            }
            _ => {}
        }
    });
}
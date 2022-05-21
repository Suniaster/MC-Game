use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
};

mod texture; // NEW!
mod voxel;
mod camera;
mod scene;
mod entity;
mod cube;
mod quad;
mod vertex;
mod grid;
use scene::*;

pub struct ViewActions{
    state: State
}

pub struct ViewObjectInfo{
    pub position: [f32; 3],
    pub color: [f32; 3],
    pub size: [f32; 3],
    pub id: u32
}

impl ViewActions{
    pub fn create_cube(&mut self, obj: &ViewObjectInfo)-> u32{
        let mesh = quad::hexagon::HexagonMesh::new(
            cgmath::Vector3::from(obj.position), 
            cgmath::Vector3::from(obj.size)/2., 
        );
        let new_ent = entity::SceneEntity::new(
            &self.state.device, 
            cgmath::Vector3::from(obj.position),
            &mesh
        );
        let id = new_ent.id;
        self.state.entities.insert(id, new_ent);
        return id;
    }

    pub fn update_cube(&mut self, obj: &ViewObjectInfo){
        let entity = self.state.entities.get_mut(&obj.id).unwrap();
        entity.update_pos(&self.state.queue, cgmath::Vector3::from(obj.position));
    }
}

pub trait ViewController{
    fn on_update(&mut self, a:&mut ViewActions, b:std::time::Duration) -> ();
    fn on_keybord_input(&mut self, a: &mut ViewActions, b:VirtualKeyCode, c:ElementState) -> ();
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
            Event::RedrawRequested(window_id) if window_id == window.id() => {
                let now = std::time::Instant::now();
                let dt = now - last_render_time;
                last_render_time = now;
                actions.state.update(dt);
                controller.on_update(&mut actions, dt);
            }
            _ => {}
        }
    });
}
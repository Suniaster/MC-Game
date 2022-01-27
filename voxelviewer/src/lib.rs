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

use world::scene::GameScene;

pub struct ViewActions{
    state: State
}

impl ViewActions{
    pub fn create_cube(&mut self, position: [f32;3], _color: [f32;3])->u32{
        let new_ent = entity::SceneEntity::new(
            &self.state.device, 
            cgmath::Vector3::from(position)
        );
        let id = new_ent.id;
        self.state.entities.insert(id, new_ent);
        id
    }

    pub fn update_cube(&mut self, idx: u32, position: [f32; 3]){
        let entity = self.state.entities.get_mut(&idx).unwrap();
        entity.update_pos(&self.state.queue, cgmath::Vector3::from(position));
    }
}

pub struct ViewController{
    pub on_update: fn(&mut ViewActions,&mut GameScene, dt:std::time::Duration) -> (),
    pub on_keybord_input: fn(&mut ViewActions, &mut GameScene, VirtualKeyCode, ElementState) -> (),
}

impl ViewController{
    pub fn new() -> ViewController{
        ViewController{
            on_update: |_,_,_|{},
            on_keybord_input: |_,_,_,_|{},
        }
    }
}

pub fn main(controller: ViewController, game_scene: world::scene::GameScene) {
    env_logger::init();
    let event_loop = EventLoop::new();
    let title = env!("CARGO_PKG_NAME");
    let window = winit::window::WindowBuilder::new()
        .with_title(title)
        .build(&event_loop)
        .unwrap();


    let mut game_scene = game_scene;
    let mut actions = ViewActions{state: pollster::block_on(State::new(&window))};

    let mut last_render_time = std::time::Instant::now();
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
                        (controller.on_keybord_input)(&mut actions, &mut game_scene, *key, *state);
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
                (controller.on_update)(&mut actions, &mut game_scene, dt);
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
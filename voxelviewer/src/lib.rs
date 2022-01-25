use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
};

mod texture; // NEW!
mod voxel;
mod camera;
mod scene;
use scene::*;

use wgpu::util::DeviceExt;
use world::scene::GameScene;

pub struct ViewActions{
    state: State
}

impl ViewActions{
    pub fn create_cube(&mut self, position: [f32;3], color: [f32;3])->u32{
        self.state.instances.push(voxel::Instance::new(
            cgmath::Vector3::from(position),
            color
        ));
        self.update_entire_cube_buffer();
        self.state.instances.last().unwrap().idx
    }

    pub fn update_cube(&mut self, idx: u32, position: [f32; 3]){
        let size_raw = std::mem::size_of::<voxel::InstanceRaw>() as wgpu::BufferAddress;

        let instance_index = self.state.instances.iter().position(|r| r.idx == idx).unwrap();
        let instance = &mut self.state.instances[instance_index];
        instance.position = cgmath::Vector3::from(position);

        self.state.queue.write_buffer(
            &self.state.instance_buffer, 
            size_raw * instance_index as wgpu::BufferAddress, 
            bytemuck::cast_slice(&[instance.to_raw()])
        );
    }

    #[inline(always)]
    fn update_entire_cube_buffer(&mut self){
        let instance_data = self.state.instances.iter().map(voxel::Instance::to_raw).collect::<Vec<_>>();
        let instance_buffer = self.state.device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Instance Buffer"),
                contents: bytemuck::cast_slice(&instance_data),
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            }
        ); 
        self.state.instance_buffer = instance_buffer;   
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
    let mut loop_dt = std::time::Instant::now();
    event_loop.run(move |event, _, control_flow| {
        let now = std::time::Instant::now();
        let dt = now - loop_dt;
        loop_dt = now;
        (controller.on_update)(&mut actions, &mut game_scene, dt);

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
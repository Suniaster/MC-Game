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
use view_actions::*;

pub mod view_actions;

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
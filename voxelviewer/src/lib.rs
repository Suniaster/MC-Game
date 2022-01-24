use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
};

mod texture; // NEW!
mod voxel;
mod camera;
mod scene;
use scene::*;
// use voxel::*;


pub struct ViewActions{
    state: State
}

pub struct ViewController{
    pub on_update: fn(actions: &ViewActions, dt: std::time::Duration) -> (),
}

impl ViewController{
    pub fn new() -> ViewController{
        ViewController{
            on_update: |_,_|{},
        }
    }
}

pub fn main(controller: ViewController) {
    env_logger::init();
    let event_loop = EventLoop::new();
    let title = env!("CARGO_PKG_NAME");
    let window = winit::window::WindowBuilder::new()
        .with_title(title)
        .build(&event_loop)
        .unwrap();

    let mut actions = ViewActions{state: pollster::block_on(State::new(&window))};

    let mut last_render_time = std::time::Instant::now();
    let mut loop_dt = std::time::Instant::now();
    event_loop.run(move |event, _, control_flow| {
        let now = std::time::Instant::now();
        let dt = now - loop_dt;
        loop_dt = now;
        (controller.on_update)(&actions, dt);

        *control_flow = ControlFlow::Poll;
        match event {
            Event::MainEventsCleared => window.request_redraw(),
            Event::DeviceEvent {
                ref event,
                .. // We're not using device_id currently
            } => {
                actions.state.input(event);
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
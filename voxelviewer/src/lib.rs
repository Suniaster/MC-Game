#![allow(dead_code)]
use std::{sync::{Mutex, Arc}};

use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
};

mod texture; 
mod camera;
mod scene;
mod geometry;
pub mod screen_text;
mod draw;
pub mod view_system;

use scene::*;

pub mod view_actions;

pub struct ScreenView {
    pub state: State,
    window: winit::window::Window,
}

pub fn create_screen() -> (ScreenView, EventLoop<()>) {
    env_logger::init();
    let event_loop = EventLoop::new();
    let title = env!("CARGO_PKG_NAME");
    let window = winit::window::WindowBuilder::new()
        .with_title(title)
        .build(&event_loop)
        .unwrap();

    let state =  pollster::block_on(State::new(&window));

    let screen = ScreenView {
        state,
        window,
    };

    (screen, event_loop)
}

pub fn start(
    mut world: specs::World, 
    mut dispatcher: specs::Dispatcher<'static, 'static>, 
    screen_arc: Arc<Mutex<ScreenView>>, 
    event_loop: EventLoop<()>
) {
    let mut last_render_time = std::time::Instant::now();
    event_loop.run(move |event, _, control_flow| {
        let mut screen = screen_arc.lock().unwrap();

        *control_flow = ControlFlow::Poll;
        match event {
            Event::MainEventsCleared =>  screen.window.request_redraw(),
            Event::DeviceEvent {
                ref event,
                .. // We're not using device_id currently
            } => {
                screen.state.input(event);
            }
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == screen.window.id() => {
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
                        screen.state.resize(*physical_size);
                    }
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        screen.state.resize(**new_inner_size);
                    }
                    _ => {}
                }
            }
            Event::RedrawRequested(window_id) if window_id == screen.window.id() => {
                let now = std::time::Instant::now();
                let dt = now - last_render_time;
                last_render_time = now;
                screen.state.update(dt);
                drop(screen); // Drop screen to clear mutex
                dispatcher.dispatch(&mut world);
            }
            _ => {}
        }
    });
}
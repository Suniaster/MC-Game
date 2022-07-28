use std::sync::Mutex;

use plugins::{Plugin, PluginSytem, App};
use specs::{System, WorldExt, Read, WriteExpect};
use winit::{event_loop::{EventLoop, ControlFlow}, dpi::{LogicalSize, PhysicalSize}, event::{Event, WindowEvent}};
use winit::event::DeviceEvent;

use crate::scene::State;

/**** Event Buffers ****/
#[derive(Default)]
pub struct WindowResizeBuffer {
    pub events: Vec<PhysicalSize<u32>>,
}

#[derive(Default)]
pub struct DeviceEventBuffer {
    pub events: Vec<DeviceEvent>,
}
/**** END - Event Buffers ****/


struct WindowSystem;
impl<'a> System<'a> for WindowSystem {
    type SystemData = (
        WriteExpect<'a, Mutex<State>>,
        Read<'a, WindowResizeBuffer>,
    );

    fn run(&mut self, (mutex_state, win_evs): Self::SystemData) {
        let mut state = mutex_state.lock().unwrap();
        for ev in win_evs.events.iter() {
            println!("WindowEvent: {:?}", ev);
            state.resize(*ev);
        }
        drop(state);
    }

}

impl PluginSytem<'_> for WindowSystem {
    fn name(&self) -> &'static str {
        "window_system"
    }
}

pub struct WindowPlugin;
impl Plugin for WindowPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(WindowSystem);
        app.add_resource(WindowResizeBuffer::default());
        app.add_resource(DeviceEventBuffer::default());
    }

    fn before_run(&self, app: &mut App<'static>) {
        env_logger::init();
        let event_loop = EventLoop::new();
        let title = env!("CARGO_PKG_NAME");
        let window = winit::window::WindowBuilder::new()
            .with_title(title)
            .with_inner_size(LogicalSize::new(1024.0, 768.0))
            .build(&event_loop)
            .unwrap();
        
        let state =  pollster::block_on(State::new(&window));
        let m = Mutex::new(state);
        app.add_resource(m);

        let mut app = std::mem::replace(app, App::new());

        event_loop.run(move |event, _, control_flow|{

            *control_flow = ControlFlow::Poll;
            match event {
                Event::MainEventsCleared =>  window.request_redraw(),
                Event::DeviceEvent {
                    event,
                    .. // We're not using device_id currently
                } => {
                    let mut buffer = app.world.write_resource::<DeviceEventBuffer>();
                    buffer.events.push(event);
                }
                Event::WindowEvent {
                    ref event,
                    window_id,
                } if window_id == window.id() => {
                    
                    match event {
                        WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                        WindowEvent::Resized(physical_size) => {
                            let mut buffer = app.world.write_resource::<WindowResizeBuffer>();
                            buffer.events.push(*physical_size);
                        }
                        _ => {}
                    }
                }
                Event::RedrawRequested(window_id) if window_id == window.id() => {
                    app.run_once();

                    // Clear buffers
                    let mut buffer = app.world.write_resource::<WindowResizeBuffer>();
                    let mut device_buffer = app.world.write_resource::<DeviceEventBuffer>();
                    
                    buffer.events.clear();
                    device_buffer.events.clear();
                }
                _ => {}
            }
        });
    }
}



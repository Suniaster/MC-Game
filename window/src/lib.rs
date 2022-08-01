use std::sync::{Arc};

use plugins::{Plugin, PluginSytem, App};
use specs::{System, WorldExt, Write};
use winit::{event_loop::{EventLoop, ControlFlow}, dpi::{LogicalSize, PhysicalSize}, event::{Event, WindowEvent}, window::Window};
use winit::event::DeviceEvent;

pub mod logs;

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

/**** Timers ******/

#[derive(Default)]
pub struct DeltaTime(f32);

#[derive(Default)]
pub struct FrameCount(u32);

/**** END - Timers ******/


struct WindowSystem{
    last_update_time: std::time::Instant,
}
impl Default for WindowSystem {
    fn default() -> Self {
        Self {
            last_update_time: std::time::Instant::now(),
        }
    }
}

impl<'a> System<'a> for WindowSystem {
    type SystemData = (
        Write<'a, DeltaTime>,
        Write<'a, FrameCount>,
    );

    fn run(&mut self, (mut dt, mut frame_c): Self::SystemData) {
        let delta = self.last_update_time.elapsed().as_secs_f32();
        self.last_update_time = std::time::Instant::now();
        *dt = DeltaTime(delta);
        *frame_c = FrameCount(frame_c.0 + 1);
    }
}

impl PluginSytem<'_> for WindowSystem {
    fn name(&self) -> &'static str {
        "window_system"
    }
}

#[derive(Default)]
pub struct WindowPlugin{
    pub event_loop: Option<EventLoop<()>>,
    pub window: Option<Arc<Window>>,
}

impl Plugin for WindowPlugin {
    fn build(&mut self, app: &mut App) {
        app.add_system(WindowSystem::default());
        app.add_resource(WindowResizeBuffer::default());
        app.add_resource(DeviceEventBuffer::default());
        app.add_resource(DeltaTime::default());
        app.add_resource(FrameCount::default());

        let title = env!("CARGO_PKG_NAME");
        let event_loop = EventLoop::new();
        let window = winit::window::WindowBuilder::new()
            .with_title(title)
            .with_inner_size(LogicalSize::new(1024.0, 768.0))
            .build(&event_loop)
            .unwrap();
        
        let window = Arc::new(window);
        app.add_resource(window.clone());
        self.event_loop = Some(event_loop);
        self.window = Some(window);
    }

    fn before_run(&mut self, app: &mut App<'static>) {
        env_logger::init();

        let window = std::mem::replace(&mut self.window, None).unwrap();
        let event_loop = std::mem::replace(&mut self.event_loop, None).unwrap();
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

                    let mut buffer = app.world.write_resource::<WindowResizeBuffer>();
                    buffer.events.clear();
                    let mut buffer = app.world.write_resource::<DeviceEventBuffer>();
                    buffer.events.clear();
                }
                _ => {}
            }
        });
    }
}



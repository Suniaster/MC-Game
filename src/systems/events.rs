use std::sync::Arc;
use std::sync::Mutex;

use nalgebra::Point3;
use specs::prelude::*;

use crate::MultiThread;

trait ViewSystemEventListener {
    fn entity_created(&mut self, grid: &CreateGridPayload){}
}


type GridDesc = Arc<Vec<Vec<Vec<bool>>>>;
struct CreateGridPayload{
    id: u32, 
    grid: GridDesc,
    position: Point3<f32>,
    cube_size: f32,
}


enum ViewEventPayload {
    CreateGridPayload(CreateGridPayload),
}

struct ViewEventManager{
    events_buffer: Vec<ViewEventPayload>,
    listeners: Vec<Box<dyn ViewSystemEventListener + Sync + Send + 'static>>,
}

impl ViewEventManager {
    pub fn new() -> Self {
        Self {
            events_buffer: Vec::new(),
            listeners: Vec::new(),
        }
    }

    pub fn subscribe(&mut self, listener: Box<dyn ViewSystemEventListener + Sync + Send + 'static>) {
        self.listeners.push(listener);
    }

    pub fn notify_all(&mut self) {
        for listener in self.listeners.iter_mut() {
            for payload in self.events_buffer.iter() {
                match payload {
                    ViewEventPayload::CreateGridPayload(grid) => {
                        listener.entity_created(grid);
                    }
                }
            }
        }
    }

    pub fn add_event(&mut self, payload: ViewEventPayload) {
        self.events_buffer.push(payload);
    }

    pub fn clear_events(&mut self) {
        self.events_buffer.clear();
    }
}

struct ViewEventsSystem;
impl <'a> System <'a> for ViewEventsSystem {
    type SystemData =
        WriteExpect<'a ,MultiThread<ViewEventManager>>
    ;

    fn run(&mut self, mut event_manager: Self::SystemData) {
        let mut em = event_manager.lock().unwrap();
        em.notify_all();
        em.clear_events();
    }
}

impl ViewEventsSystem {
    pub fn create_manager() -> MultiThread<ViewEventManager> {
        Arc::new(Mutex::new(ViewEventManager::new()))
    }
}
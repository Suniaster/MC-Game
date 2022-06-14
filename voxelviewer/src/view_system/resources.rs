use winit::event::DeviceEvent;



#[derive(Default)]
pub struct DeviceEventBuffer {
    pub events: Vec<DeviceEvent>,
}
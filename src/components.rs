use nalgebra::Point3;
use specs::{Component, VecStorage};


pub struct RenderComponent {
    pub is_rendered: bool,
    pub obj_id: u32
}

impl Component for RenderComponent {
    type Storage = VecStorage<Self>;
}

pub struct PositionComponent(pub Point3<f32>);

impl Component for PositionComponent {
    type Storage = VecStorage<Self>;
}


type Mat3 = Vec<Vec<Vec<bool>>>;
pub struct GridDescriptorComponent{
    pub desc: Mat3, 
    pub cube_size: f32
}

impl Component for GridDescriptorComponent {
    type Storage = VecStorage<Self>;
}

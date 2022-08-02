use nalgebra::Point3;
use specs::{Component, VecStorage};



pub struct PositionComponent(pub Point3<f32>);
impl Component for PositionComponent {type Storage = VecStorage<Self>;}
impl PositionComponent {pub fn new(position: Point3<f32>) -> Self {Self(position)}}
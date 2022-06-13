use nalgebra::{UnitComplex, Point3};
use specs::{Component, VecStorage};

pub struct LookingDirection {
    yaw: UnitComplex<f32>,
    pitch: UnitComplex<f32>,
}
impl Component for LookingDirection {type Storage = specs::VecStorage<Self>;}

pub struct PositionComponent(pub Point3<f32>);
impl Component for PositionComponent {type Storage = VecStorage<Self>;}
impl PositionComponent {pub fn new(position: Point3<f32>) -> Self {Self(position)}}
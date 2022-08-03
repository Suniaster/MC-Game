use nalgebra::Point3;
use plugins::{Plugin, App};
use specs::{Component, VecStorage};

pub mod events;

pub struct PositionComponent(pub Point3<f32>);
impl Component for PositionComponent {type Storage = VecStorage<Self>;}
impl PositionComponent {pub fn new(position: Point3<f32>) -> Self {Self(position)}}


pub struct CommonComponentsPlugin;
impl Plugin for CommonComponentsPlugin {
    fn build(&mut self, app: &mut App) {
        app.add_component_storage::<PositionComponent>();
    }
}
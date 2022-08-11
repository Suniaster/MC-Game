use nalgebra::{Point3, Vector3};
use plugins::{Plugin, App};
use specs::{Component, VecStorage, HashMapStorage};

pub mod events;

pub struct PositionComponent(pub Point3<f32>);
impl Component for PositionComponent {type Storage = VecStorage<Self>;}
impl PositionComponent {pub fn new(position: Point3<f32>) -> Self {Self(position)}}

pub struct VelocityComponent(pub Vector3<f32>);
impl Component for VelocityComponent { type Storage = VecStorage<Self>;}

#[derive(Default)]
pub struct AddRigidBodyCubeFlag(pub f32);
impl Component for AddRigidBodyCubeFlag { type Storage = HashMapStorage<Self>;}


pub struct CommonComponentsPlugin;
impl Plugin for CommonComponentsPlugin {
    fn build(&mut self, app: &mut App) {
        app.add_component_storage::<PositionComponent>();
        app.add_component_storage::<VelocityComponent>();
        app.add_component_storage::<AddRigidBodyCubeFlag>()
    }
}
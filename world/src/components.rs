use ecs::vec_storage::GenerationalIndexArray;
use ecs::GenerationalIndex;

use voxelviewer::view_actions::ViewObjectInfo;

use cgmath::Vector3;

// Meta data

pub type ComponentMap<T> = GenerationalIndexArray<T>;
pub type EntityIdx = GenerationalIndex;

// Components Definition
pub struct PhysicsComponent {
    pub vel: Vector3<f32>,
    pub accel: Vector3<f32>,
}

impl PhysicsComponent {
    pub fn new(vel: Vector3<f32>, accel: Vector3<f32>) -> PhysicsComponent {
        return PhysicsComponent { vel, accel };
    }

    pub fn new_random() -> PhysicsComponent {
        return PhysicsComponent::new(Vector3::new(0., 0., 0.), Vector3::new(0., -1., 0.));
    }
}

pub struct PositionComponent(pub Vector3<f32>);
pub struct SizeComponent(pub f32, pub f32);

pub struct RenderComponent{pub obj: ViewObjectInfo}

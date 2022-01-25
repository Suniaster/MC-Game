use ncollide2d::math::Vector;

use ecs::vec_storage::GenerationalIndexArray;
use ecs::GenerationalIndex;

// Meta data

pub type ComponentMap<T> = GenerationalIndexArray<T>;
pub type EntityIdx = GenerationalIndex;

// Components Definition
pub struct PhysicsComponent {
    pub vel: Vector<f64>,
    pub accel: Vector<f64>,
}

impl PhysicsComponent {
    pub fn new(vel: Vector<f64>, accel: Vector<f64>) -> PhysicsComponent {
        return PhysicsComponent { vel, accel };
    }

    pub fn new_random() -> PhysicsComponent {
        return PhysicsComponent::new(Vector::new(0., 0.), Vector::new(0., -1.));
    }
}

pub struct PositionComponent(pub Vector<f64>);
pub struct SizeComponent(pub f64, pub f64);
pub struct RenderComponent{pub cube_idx: u32}

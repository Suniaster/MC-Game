use nalgebra::Vector3;
use specs::Component;

pub struct PhysicsComponent {
    pub velocity: Vector3<f32>,
    pub acceleration: Vector3<f32>,
}

impl PhysicsComponent {
    pub fn new(velocity: Vector3<f32>, acceleration: Vector3<f32>) -> Self {
        Self {
            velocity,
            acceleration,
        }
    }

    pub fn default() -> Self {
        Self {
            velocity: Vector3::new(0.0, 0.0, 0.0),
            acceleration: Vector3::new(0.0, 0.0, 0.0),
        }
    }
}

impl Component for PhysicsComponent {
    type Storage = specs::VecStorage<Self>;
}

use crate::ecs::Component;
use cgmath::Vector3;

pub struct PositionComponent {
    position: Vector3<f32>,
}

impl PositionComponent {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            position: Vector3::new(x, y, z),
        }
    }
}

impl Component for PositionComponent {}

use cgmath::Vector3;

use crate::ecs::Component;

pub struct Position {
    pub position: Vector3<f32>,
    pub rotation: Vector3<f32>,
}

impl Position {
    pub fn new(position: Vector3<f32>, rotation: Vector3<f32>) -> Self {
        Self { position, rotation }
    }
}

impl Component for Position {}

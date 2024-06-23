use cgmath::Vector3;

use crate::{Asset, ecs::Component};

pub struct Sprite {
    pub image: Asset
}

impl Position {
    pub fn new(position: Vector3<f32>, rotation: Vector3<f32>) -> Self {
        Self { position, rotation }
    }
}

impl Component for Position {}

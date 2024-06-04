use crate::ecs::System;

pub struct CameraSystem {}

impl CameraSystem {
    pub fn new() -> Self {
        Self {}
    }
}

impl System for CameraSystem {
    // const TYPE_NAME: &'static str = "CameraComponent";
}

impl Default for CameraSystem {
    fn default() -> Self {
        CameraSystem::new()
    }
}

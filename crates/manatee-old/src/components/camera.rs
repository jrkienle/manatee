use crate::ecs::Component;

pub struct Camera {
    pub is_main: bool,
}

impl Camera {
    pub fn new() -> Self {
        Self { is_main: true }
    }
}

impl Component for Camera {}

impl Default for Camera {
    fn default() -> Self {
        Camera::new()
    }
}

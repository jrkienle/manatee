use crate::ecs::Component;

pub struct CameraComponent {
    pub main: bool,
}

impl CameraComponent {
    pub fn new() -> Self {
        Self { main: true }
    }
}

impl Component for CameraComponent {
    const TYPE_NAME: &'static str = "CameraComponent";
}

impl Default for CameraComponent {
    fn default() -> Self {
        CameraComponent::new()
    }
}

use crate::component::Component;

pub struct CameraComponent {}

impl CameraComponent {
    pub fn new() -> Self {
        Self {}
    }
}

impl Component for CameraComponent {}

impl Default for CameraComponent {
    fn default() -> Self {
        CameraComponent::new()
    }
}

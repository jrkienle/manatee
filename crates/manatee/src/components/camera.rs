use crate::ecs::Component;

pub enum CameraMode {
    Camera2D,
    Camera3D,
}

pub struct Camera {
    pub is_main: bool,
    pub mode: CameraMode,
}

impl Camera {
    pub fn new(mode: CameraMode) -> Self {
        Self {
            is_main: true,
            mode,
        }
    }
}

impl Component for Camera {}

impl Default for Camera {
    fn default() -> Self {
        Camera::new(CameraMode::Camera3D)
    }
}

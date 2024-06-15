use crate::ecs::Component;

pub struct WgpuTutorial {}

impl WgpuTutorial {
    pub fn new() -> Self {
        Self {}
    }
}

impl Component for WgpuTutorial {}

impl Default for WgpuTutorial {
    fn default() -> Self {
        WgpuTutorial::new()
    }
}

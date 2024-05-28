use std::sync::Arc;

use winit::{event_loop::EventLoop, window::Window};

use crate::{gpu::Gpu, scene::Scene, scene_manager::SceneManager, window_state::WindowState};

pub struct Game {
    event_loop: EventLoop<()>,
    gpu: Option<Arc<Gpu>>,
    scene_manager: SceneManager,
    window: Option<Arc<Window>>,
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}

impl Game {
    pub fn new() -> Self {
        let event_loop = EventLoop::new().unwrap();
        Self {
            event_loop,
            gpu: None,
            scene_manager: SceneManager::new(),
            window: None,
        }
    }

    pub fn load_scene(&mut self, scene: Scene) {
        self.scene_manager.load_scene(scene);
    }

    pub fn run(self) {
        let mut window_state = WindowState::new(self.window, self.gpu, self.scene_manager);
        self.event_loop.run_app(&mut window_state).unwrap();
    }
}

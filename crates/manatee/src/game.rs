use std::sync::Arc;

use winit::{event_loop::EventLoop, window::Window};

use crate::{gpu::Gpu, window_state::WindowState};

pub struct Game {
    event_loop: EventLoop<()>,
    gpu: Option<Arc<Gpu>>,
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
            window: None,
        }
    }

    pub fn run(self) {
        println!("Running your game");
        let mut window_state = WindowState::new(self.window, self.gpu);
        self.event_loop.run_app(&mut window_state).unwrap();
    }
}

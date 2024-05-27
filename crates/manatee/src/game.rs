use std::sync::Arc;

use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    window::{Window, WindowId},
};

use crate::{gpu::Gpu, scene::Scene};

// Game WindowState Object (wrapper around winit)
struct GameWindowState {
    gpu: Option<Arc<Gpu>>,
    window: Option<Arc<Window>>,
}

impl ApplicationHandler for GameWindowState {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        println!("Event Loop Resumed");
        let window = Arc::new(
            event_loop
                .create_window(Window::default_attributes())
                .unwrap(),
        );
        println!("Window Created");

        let gpu = Arc::new(Gpu::new(Arc::clone(&window)));
        println!("GPU Created");

        self.window = Some(window);
        println!("Window Set");

        self.gpu = Some(gpu);
        println!("GPU Set")
    }
    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        // This will probably need to be a match after the app grows a bit
        if event == WindowEvent::CloseRequested {
            event_loop.exit();
        }
    }
}

// Main Game object
pub struct Game {
    current_scene: Option<Scene>,
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
            current_scene: None,
            event_loop,
            gpu: None,
            window: None,
        }
    }

    pub fn run(self) {
        println!("Running your game");
        let mut game_window_state = GameWindowState {
            // current_scene: self.current_scene.unwrap(),
            gpu: self.gpu,
            window: self.window,
        };
        self.event_loop.run_app(&mut game_window_state).unwrap();
    }

    pub fn load_scene(&mut self, scene: Scene) {
        println!("Loading Scene");
        self.current_scene = Some(scene);
        println!("Scene Loaded");
    }
}

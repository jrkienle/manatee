use crate::gpu::Gpu;
use aquarium::{App, WindowParams};
use cgmath::Vector2;
use pollster::block_on;

pub struct Game {
    app: App,
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}

impl Game {
    pub fn new() -> Self {
        let app = App::new();
        Self { app }
    }

    pub fn run(self) {
        self.app.run(|app_ctx| {
            let window_params = WindowParams { title: "Manatee game", ..Default::default() };
            app_ctx.new_window(window_params, |window_ctx| {
                let mut gpu = block_on(Gpu::new(window_ctx.window));
                gpu.resize(window_ctx.window);
                let _ = gpu.render();
            });
        })
    }
}

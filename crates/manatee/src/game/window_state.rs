use std::sync::Arc;

use cgmath::Vector2;
use pollster::block_on;
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::ActiveEventLoop,
    window::{Window, WindowId},
};

use crate::graphics::Gpu;
use super::SceneManager;

pub struct WindowState {
    pub(crate) gpu: Option<Arc<Gpu>>,
    pub(crate) scene_manager: SceneManager,
    pub(crate) window: Option<Arc<Window>>,
}

impl WindowState {
    pub fn new() -> Self {
        Self {
            gpu: None,
            scene_manager: SceneManager::new(),
            window: None,
        }
    }
}

impl ApplicationHandler for WindowState {
    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        self.window.as_ref().unwrap().request_redraw();
    }

    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = Arc::new(
            event_loop
                .create_window(Window::default_attributes())
                .unwrap(),
        );

        // Wgpu uses async functions to maintain better web compatibility. Unfortunately, Winit
        // doesn't have native support for async functions, so I have to use pollster to block
        // the thread until the Wgpu has finished setting up access to the machine's GPU
        let gpu = Arc::new(block_on(Gpu::new(window.clone())));

        // Every time the window resumes, I need to call gpu.resume to configure our surface and
        // ensure it's rendering into the correct window handle
        gpu.resume(window.clone());

        self.window = Some(window);
        self.gpu = Some(gpu);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        // TODO: Should the functionality inside of this match statement be abstracted into
        // individual struct methods for each event?
        match event {
            WindowEvent::CloseRequested => {
                // TODO: I may need to add additional logic here if I allow the engine to create child
                // windows (this is probably a SUPER useful edge case for building an editor app)
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                // let (systems, mut ctx) = Context::new(self);
                let gpu = self.gpu.clone().unwrap();
                self.scene_manager
                    .active_scene
                    .as_mut()
                    .unwrap()
                    .render(gpu);

                // for (_, system) in systems.systems.iter_mut() {
                //     system.get_mut().on_update(&mut ctx);
                // }

                // for (_, system) in ctx() {
                //     let _foo = &mut system.get_mut().on_update(&mut ctx);
                // }
                // self.gpu.as_ref().unwrap().render_frame();
                // Once I've rendered the frame, this exact event gets kicked off again to create
                // an infinite game loop!
                self.window.as_ref().unwrap().request_redraw();
            }
            WindowEvent::Resized(physical_size) => {
                let width = physical_size.width.max(1);
                let height = physical_size.height.max(1);

                let gpu = self.gpu.as_ref().unwrap();
                gpu.resize(Vector2 {
                    x: width,
                    y: height,
                })
            }
            _ => (),
        }
    }
}

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

        let gpu = Arc::new(pollster::block_on(Gpu::new(window.clone())));
        println!("GPU Created");

        self.window = Some(window);
        println!("Window Set");

        self.gpu = Some(gpu);
        println!("GPU Set");

        let device = self.gpu.as_ref().unwrap().device();
        let queue = self.gpu.as_ref().unwrap().queue();
        let surface = self.gpu.as_ref().unwrap().surface();

        let output = surface.get_current_texture().unwrap();
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
        println!("Current Texture Found");
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        {
            let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });
        }
    
        // submit will accept anything that implements IntoIter
        queue.submit(std::iter::once(encoder.finish()));
        output.present();
        println!("Rendered");
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

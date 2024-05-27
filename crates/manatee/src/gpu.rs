use std::sync::Arc;
use wgpu::Surface;
use winit::window::Window;

pub struct Gpu {
    surface: Surface<'static>,
}

impl Gpu {
    pub fn new(window: Arc<Window>) -> Self {
        let instance = wgpu::Instance::default();
        let size = window.inner_size();
        let surface = instance.create_surface(window).unwrap();

        Self { surface }
    }
}

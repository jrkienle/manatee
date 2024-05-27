use std::sync::Arc;
use wgpu::{Device,Queue,Surface};
use winit::window::Window;

pub struct Gpu {
    device: Device,
    queue: Queue,
    surface: Surface<'static>,
}

impl Gpu {
    pub async fn new(window: Arc<Window>) -> Self {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::default());
        let surface = instance.create_surface(window.clone()).unwrap();
        let adapter = instance.request_adapter(&wgpu::RequestAdapterOptionsBase::default()).await
        .expect("Invalid Graphics Backend!");

        let (device, queue) = adapter
        .request_device(&wgpu::DeviceDescriptor::default(), None)
        .await
        .unwrap();

        // TODO: Move this to resize method
        let config = surface.get_default_config(&adapter, window.inner_size().width, window.inner_size().height).unwrap();
        surface.configure(&device, &config);

        Self { device, queue, surface }
    }

    pub fn render_frame() {}

    pub fn device(&self) -> &Device {
        &self.device
    }

    pub fn queue(&self) -> &Queue {
        &self.queue
    }

    pub fn surface(&self) -> &Surface<'static> {
        &self.surface
    }
}
 
use std::sync::Arc;

use cgmath::Vector2;
use wgpu::{Adapter, Device, Queue, Surface};
use winit::window::Window;

pub struct Gpu {
    pub adapter: Adapter,
    pub device: Device,
    pub queue: Queue,
    pub surface: Surface<'static>,
}

impl Gpu {
    pub async fn new(window: Arc<Window>) -> Self {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::default());
        let surface = instance.create_surface(window.clone()).unwrap();
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptionsBase::default())
            .await
            .expect("Invalid Graphics Backend!");

        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor::default(), None)
            .await
            .unwrap();

        Self {
            adapter,
            device,
            queue,
            surface,
        }
    }

    pub fn resume(&self, window: Arc<Window>) {
        self.resize(Vector2 {
            x: window.inner_size().width,
            y: window.inner_size().height,
        })
    }

    pub fn resize(&self, size: Vector2<u32>) {
        let config = self
            .surface
            .get_default_config(&self.adapter, size.x, size.y)
            .unwrap();
        self.surface.configure(&self.device, &config);
    }
}

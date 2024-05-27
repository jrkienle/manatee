use std::{iter::once, sync::Arc};

use cgmath::Vector2;
use wgpu::{
    Adapter, Color, Device, LoadOp, Operations, Queue, RenderPassColorAttachment,
    RenderPassDescriptor, StoreOp, Surface,
};
use winit::window::Window;

pub struct Gpu {
    adapter: Adapter,
    device: Device,
    queue: Queue,
    surface: Surface<'static>,
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

    // TODO: This PR is already getting pretty big considering it was originally supposed to be
    // "abstract windowing and render something with the gpu". With that context in mind, I'm well
    // aware that this will only let Manatee render a big ass blue(ish) square and that this
    // function will need to be MASSIVELY abstracted to actually render a game. That's coming in
    // the next PR (just one more PR bro, one more abstraction, ONE MORE FRAMEWORK BRO I PROMISE)
    pub fn render_frame(&self) {
        let surface_texture = self.surface.get_current_texture().unwrap();
        let view = surface_texture.texture.create_view(&Default::default());
        let mut encoder = self.device.create_command_encoder(&Default::default());

        encoder.begin_render_pass(&RenderPassDescriptor {
            color_attachments: &[Some(RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: Operations {
                    load: LoadOp::Clear(Color {
                        r: 0.1,
                        g: 0.2,
                        b: 0.3,
                        a: 1.0,
                    }),
                    store: StoreOp::Store,
                },
            })],
            ..Default::default()
        });

        self.queue.submit(once(encoder.finish()));
        surface_texture.present();
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

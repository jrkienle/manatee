use super::Gpu;
use std::sync::Arc;
use wgpu::{CommandEncoder, SurfaceTexture, TextureView};

pub struct RenderTarget {
    pub encoder: CommandEncoder,
    pub surface_texture: SurfaceTexture,
    pub view: TextureView,
}

impl RenderTarget {
    pub fn new(gpu: Arc<Gpu>) -> Self {
        let surface_texture = gpu.surface.get_current_texture().unwrap();
        let view = surface_texture.texture.create_view(&Default::default());
        let encoder = gpu.device.create_command_encoder(&Default::default());

        Self {
            encoder,
            surface_texture,
            view,
        }
    }

    pub fn finish(self) {
        self.surface_texture.present();
    }
}

use std::any::Any;

use crate::{components::CameraComponent, ecs::System};
use wgpu::{Color, LoadOp, Operations, RenderPassColorAttachment, RenderPassDescriptor, StoreOp};

pub struct CameraSystem {}

impl CameraSystem {
    pub fn new() -> Self {
        Self {}
    }
}

impl System for CameraSystem {
    fn on_update(&self, ctx: &mut crate::Context) {
        let camera_component = CameraComponent::type_id();
        ctx.components.components.get()
        ctx.render_target
            .encoder
            .begin_render_pass(&RenderPassDescriptor {
                color_attachments: &[Some(RenderPassColorAttachment {
                    view: &ctx.render_target.view,
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
    }
}

impl Default for CameraSystem {
    fn default() -> Self {
        CameraSystem::new()
    }
}

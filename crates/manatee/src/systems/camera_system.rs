use crate::{components::Camera, ecs::System};
use wgpu::{Color, LoadOp, Operations, RenderPassColorAttachment, RenderPassDescriptor, StoreOp};

#[derive(Default)]
pub struct CameraSystem {}

impl System for CameraSystem {
    fn on_update(&self, ctx: &mut crate::Context) {
        let all_camera_instances = ctx.components.get_all_instances::<Camera>();

        // TODO: Figure out if I have to individually downcast_ref here or if I can do this in the
        // ComponentManager
        for (entity_id, camera_component) in all_camera_instances {
            if camera_component.downcast_ref::<Camera>().unwrap().is_main {
                println!("Main Camera Assigned to Entity {entity_id}")
            }
        }

        let render_pipeline_layout = ctx.gpu.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });
 

        let render_pipeline = ctx.gpu.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("why"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main", // 1.
                buffers: &[], // 2.
                compilation_options: wgpu::PipelineCompilationOptions::default()
            },
            fragment: None,
        });

        // TODO: I should probably abstract the raw WGPU code away somewhere to make the DX
        // actually good, because this is miserable for most engineers and way too powerful for
        // probably 99% of system use cases (I'll make sure to export it still if needed)
        let render_pass = ctx.render_target
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

        render_pass.set_pipeline(&render_pipeline); // 2.
        render_pass.draw(0..3, 0..1); // 3.
    }
}

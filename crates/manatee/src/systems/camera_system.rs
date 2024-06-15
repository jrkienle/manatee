use crate::{components::Camera, ecs::System};
use wgpu::{Color, LoadOp, Operations, RenderPassColorAttachment, RenderPassDescriptor, StoreOp};

#[derive(Default)]
pub struct CameraSystem {}

// TODO: Figure out how the fuck to do the following:
// 1. Pull 99% of this shit out of here
// 2. Abstract all of the rendering / shader things to make the DX actually easy
// 3. Figure out how to move the position of rendered entities / cameras in the world
// I don't know what I'm doing, I haven't even had time to work on this for like a week, help
impl System for CameraSystem {
    fn on_update(&self, ctx: &mut crate::Context) {
        let all_camera_instances = ctx.components.get_all_instances::<Camera>();

        // TODO: Figure out if I have to individually downcast_ref here or if I can do this in the
        // ComponentManager
        for (_entity_id, camera_component) in all_camera_instances {
            if camera_component.downcast_ref::<Camera>().unwrap().is_main {
                // TODO: Figure out a use for a main camera lol
                // println!("Main Camera Assigned to Entity {entity_id}")
            }
        }

        let surface_caps = ctx.gpu.surface.get_capabilities(&ctx.gpu.adapter);
        // Shader code in this tutorial assumes an sRGB surface texture. Using a different
        // one will result in all the colors coming out darker. If you want to support non
        // sRGB surfaces, you'll need to account for that when drawing to the frame.
        let surface_format = surface_caps
            .formats
            .iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(surface_caps.formats[0]);
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            // TODO: Ensure these are correct, maybe save window size in ctx somewhere
            width: ctx.render_target.surface_texture.texture.width(),
            height: ctx.render_target.surface_texture.texture.height(),
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        let render_pipeline_layout =
            ctx.gpu
                .device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("Render Pipeline Layout"),
                    bind_group_layouts: &[],
                    push_constant_ranges: &[],
                });

        let shader = ctx
            .gpu
            .device
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("Shader"),
                source: wgpu::ShaderSource::Wgsl(
                    // TODO: Make a standard folder for shaders to go in and create a path var
                    include_str!("../graphics/example_shader.wgsl").into(),
                ),
            });

        let render_pipeline =
            ctx.gpu
                .device
                .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                    label: Some("why"),
                    layout: Some(&render_pipeline_layout),
                    vertex: wgpu::VertexState {
                        module: &shader,
                        entry_point: "vs_main", // 1.
                        buffers: &[],           // 2.
                        compilation_options: wgpu::PipelineCompilationOptions::default(),
                    },
                    fragment: Some(wgpu::FragmentState {
                        module: &shader,
                        entry_point: "fs_main",
                        targets: &[Some(wgpu::ColorTargetState {
                            format: config.format,
                            blend: Some(wgpu::BlendState {
                                color: wgpu::BlendComponent::REPLACE,
                                alpha: wgpu::BlendComponent::REPLACE,
                            }),
                            write_mask: wgpu::ColorWrites::ALL,
                        })],
                        compilation_options: wgpu::PipelineCompilationOptions::default(),
                    }),
                    primitive: wgpu::PrimitiveState {
                        topology: wgpu::PrimitiveTopology::TriangleList,
                        strip_index_format: None,
                        front_face: wgpu::FrontFace::Ccw,
                        cull_mode: Some(wgpu::Face::Back),
                        // Setting this to anything other than Fill requires Features::POLYGON_MODE_LINE
                        // or Features::POLYGON_MODE_POINT
                        polygon_mode: wgpu::PolygonMode::Fill,
                        // Requires Features::DEPTH_CLIP_CONTROL
                        unclipped_depth: false,
                        // Requires Features::CONSERVATIVE_RASTERIZATION
                        conservative: false,
                    },
                    depth_stencil: None,
                    multisample: wgpu::MultisampleState {
                        count: 1,
                        mask: !0,
                        alpha_to_coverage_enabled: false,
                    },
                    // If the pipeline will be used with a multiview render pass, this
                    // indicates how many array layers the attachments will have.
                    multiview: None,
                });

        // TODO: I should probably abstract the raw WGPU code away somewhere to make the DX
        // actually good, because this is miserable for most engineers and way too powerful for
        // probably 99% of system use cases (I'll make sure to export it still if needed)
        let mut render_pass = ctx
            .render_target
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

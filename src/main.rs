mod context;
mod mesh4;

use anyhow::Result;
use winit::event::WindowEvent;
use zerocopy::{AsBytes, FromBytes};

use context::{Application, Ctx};

#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct CutPlane {
    normal: [f32; 4],
    distance: f32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, FromBytes, AsBytes)]
struct DrawIndirectCommand {
    vertex_count: u32,
    instance_count: u32,
    first_vertex: u32,
    first_instance: u32,
}

impl Default for DrawIndirectCommand {
    fn default() -> Self {
        Self {
            vertex_count: 0,
            instance_count: 1,
            first_vertex: 0,
            first_instance: 0,
        }
    }
}

const MAX_VERTEX_SIZE: wgpu::BufferAddress = 8192;

struct TestApp {
    render_pipeline: wgpu::RenderPipeline,
    compute_pipeline: wgpu::ComputePipeline,
    mesh: mesh4::Mesh4,
    cut_plane: CutPlane,
    simplex_count: wgpu::Buffer,
    draw_indirect_command: wgpu::Buffer,
    dst_vertices: wgpu::Buffer,
    compute_uniforms_bind_group: wgpu::BindGroup,
    compute_src_bind_group: wgpu::BindGroup,
    compute_dst_bind_group: wgpu::BindGroup,
}

impl Application for TestApp {
    fn init(ctx: &mut Ctx) -> Self {
        let vs_src = include_str!("shader.vert");
        let fs_src = include_str!("shader.frag");
        let cs_src = include_str!("shader.comp");

        let vs_spirv =
            glsl_to_spirv::compile(vs_src, glsl_to_spirv::ShaderType::Vertex)
                .unwrap();
        let fs_spirv =
            glsl_to_spirv::compile(fs_src, glsl_to_spirv::ShaderType::Fragment)
                .unwrap();
        let cs_spirv =
            glsl_to_spirv::compile(cs_src, glsl_to_spirv::ShaderType::Compute)
                .unwrap();

        let vs_data = wgpu::read_spirv(vs_spirv).unwrap();
        let fs_data = wgpu::read_spirv(fs_spirv).unwrap();
        let cs_data = wgpu::read_spirv(cs_spirv).unwrap();

        let vs_module = ctx.device.create_shader_module(&vs_data);
        let fs_module = ctx.device.create_shader_module(&fs_data);
        let cs_module = ctx.device.create_shader_module(&cs_data);

        let compute_uniform_layout = ctx.device.create_bind_group_layout(
            &wgpu::BindGroupLayoutDescriptor {
                bindings: &[wgpu::BindGroupLayoutBinding {
                    binding: 0,
                    visibility: wgpu::ShaderStage::COMPUTE,
                    ty: wgpu::BindingType::UniformBuffer { dynamic: false },
                }],
            },
        );

        let compute_src_layout = ctx.device.create_bind_group_layout(
            &wgpu::BindGroupLayoutDescriptor {
                bindings: &[
                    wgpu::BindGroupLayoutBinding {
                        binding: 0,
                        visibility: wgpu::ShaderStage::COMPUTE,
                        ty: wgpu::BindingType::UniformBuffer { dynamic: false },
                    },
                    wgpu::BindGroupLayoutBinding {
                        binding: 1,
                        visibility: wgpu::ShaderStage::COMPUTE,
                        ty: wgpu::BindingType::StorageBuffer {
                            dynamic: false,
                            readonly: true,
                        },
                    },
                    wgpu::BindGroupLayoutBinding {
                        binding: 2,
                        visibility: wgpu::ShaderStage::COMPUTE,
                        ty: wgpu::BindingType::StorageBuffer {
                            dynamic: false,
                            readonly: true,
                        },
                    },
                ],
            },
        );

        let compute_dst_layout = ctx.device.create_bind_group_layout(
            &wgpu::BindGroupLayoutDescriptor {
                bindings: &[
                    wgpu::BindGroupLayoutBinding {
                        binding: 0,
                        visibility: wgpu::ShaderStage::COMPUTE,
                        ty: wgpu::BindingType::StorageBuffer {
                            dynamic: false,
                            readonly: false,
                        },
                    },
                    wgpu::BindGroupLayoutBinding {
                        binding: 1,
                        visibility: wgpu::ShaderStage::COMPUTE,
                        ty: wgpu::BindingType::StorageBuffer {
                            dynamic: false,
                            readonly: false,
                        },
                    },
                ],
            },
        );

        let compute_pipeline_layout = ctx.device.create_pipeline_layout(
            &wgpu::PipelineLayoutDescriptor {
                bind_group_layouts: &[
                    &compute_uniform_layout,
                    &compute_src_layout,
                    &compute_dst_layout,
                ],
            },
        );

        let compute_pipeline = ctx.device.create_compute_pipeline(
            &wgpu::ComputePipelineDescriptor {
                layout: &compute_pipeline_layout,
                compute_stage: wgpu::ProgrammableStageDescriptor {
                    module: &cs_module,
                    entry_point: "main",
                },
            },
        );

        let cut_plane = CutPlane {
            normal: [0.0, 0.0, 0.0, 1.0],
            distance: 0.5,
        };

        let cut_plane_buffer = ctx
            .device
            .create_buffer_mapped(
                1,
                wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
            )
            .fill_from_slice(&[cut_plane]);

        let mesh = mesh4::tesseract(&ctx.device, 1.0);

        let simplex_count = ctx
            .device
            .create_buffer_mapped(
                1,
                wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
            )
            .fill_from_slice(&[mesh.simplex_count]);

        let draw_indirect_command = ctx
            .device
            .create_buffer_mapped(
                1,
                wgpu::BufferUsage::INDIRECT
                    | wgpu::BufferUsage::STORAGE
                    | wgpu::BufferUsage::COPY_DST
                    | wgpu::BufferUsage::MAP_READ,
            )
            .fill_from_slice(&[DrawIndirectCommand::default()]);

        let dst_vertices = ctx.device.create_buffer(&wgpu::BufferDescriptor {
            size: MAX_VERTEX_SIZE
                * std::mem::size_of::<mesh4::Vertex4>() as wgpu::BufferAddress,
            usage: wgpu::BufferUsage::STORAGE
                | wgpu::BufferUsage::VERTEX
                | wgpu::BufferUsage::MAP_READ,
        });

        let compute_uniforms_bind_group =
            ctx.device.create_bind_group(&wgpu::BindGroupDescriptor {
                layout: &compute_uniform_layout,
                bindings: &[wgpu::Binding {
                    binding: 0,
                    resource: wgpu::BindingResource::Buffer {
                        buffer: &cut_plane_buffer,
                        range: 0..std::mem::size_of_val(&cut_plane)
                            as wgpu::BufferAddress,
                    },
                }],
            });

        let compute_src_bind_group =
            ctx.device.create_bind_group(&wgpu::BindGroupDescriptor {
                layout: &compute_src_layout,
                bindings: &[
                    wgpu::Binding {
                        binding: 0,
                        resource: wgpu::BindingResource::Buffer {
                            buffer: &simplex_count,
                            range: 0..std::mem::size_of::<u32>()
                                as wgpu::BufferAddress,
                        },
                    },
                    wgpu::Binding {
                        binding: 1,
                        resource: wgpu::BindingResource::Buffer {
                            buffer: &mesh.vertex_buffer,
                            range: 0..mesh.vertex_buffer_size,
                        },
                    },
                    wgpu::Binding {
                        binding: 2,
                        resource: wgpu::BindingResource::Buffer {
                            buffer: &mesh.index_buffer,
                            range: 0..mesh.index_buffer_size,
                        },
                    },
                ],
            });

        let compute_dst_bind_group =
            ctx.device.create_bind_group(&wgpu::BindGroupDescriptor {
                layout: &compute_dst_layout,
                bindings: &[
                    wgpu::Binding {
                        binding: 0,
                        resource: wgpu::BindingResource::Buffer {
                            buffer: &draw_indirect_command,
                            range: 0..std::mem::size_of::<u32>()
                                as wgpu::BufferAddress,
                        },
                    },
                    wgpu::Binding {
                        binding: 1,
                        resource: wgpu::BindingResource::Buffer {
                            buffer: &dst_vertices,
                            range: 0..MAX_VERTEX_SIZE
                                * std::mem::size_of::<mesh4::Vertex4>()
                                    as wgpu::BufferAddress,
                        },
                    },
                ],
            });

        let render_pipeline_layout = ctx.device.create_pipeline_layout(
            &wgpu::PipelineLayoutDescriptor {
                bind_group_layouts: &[],
            },
        );

        let render_pipeline = ctx.device.create_render_pipeline(
            &wgpu::RenderPipelineDescriptor {
                layout: &render_pipeline_layout,
                vertex_stage: wgpu::ProgrammableStageDescriptor {
                    module: &vs_module,
                    entry_point: "main",
                },
                fragment_stage: Some(wgpu::ProgrammableStageDescriptor {
                    module: &fs_module,
                    entry_point: "main",
                }),
                rasterization_state: Some(wgpu::RasterizationStateDescriptor {
                    front_face: wgpu::FrontFace::Ccw,
                    cull_mode: wgpu::CullMode::Back,
                    depth_bias: 0,
                    depth_bias_slope_scale: 0.0,
                    depth_bias_clamp: 0.0,
                }),
                color_states: &[wgpu::ColorStateDescriptor {
                    format: ctx.sc_desc.format,
                    color_blend: wgpu::BlendDescriptor::REPLACE,
                    alpha_blend: wgpu::BlendDescriptor::REPLACE,
                    write_mask: wgpu::ColorWrite::ALL,
                }],
                primitive_topology: wgpu::PrimitiveTopology::TriangleList,
                depth_stencil_state: None,
                index_format: wgpu::IndexFormat::Uint16,
                vertex_buffers: &[mesh4::Vertex4::desc()],
                sample_count: 1,
                sample_mask: !0,
                alpha_to_coverage_enabled: false,
            },
        );

        TestApp {
            render_pipeline,
            compute_pipeline,
            mesh,
            cut_plane,
            draw_indirect_command,
            dst_vertices,
            simplex_count,
            compute_uniforms_bind_group,
            compute_src_bind_group,
            compute_dst_bind_group,
        }
    }

    fn resize(&mut self, _ctx: &mut Ctx) {}

    fn on_event(&mut self, _ctx: &mut Ctx, event: WindowEvent) {
        match event {
            _ => (),
        }
    }

    fn update(&mut self, _ctx: &mut Ctx) {}

    fn render(&mut self, ctx: &mut Ctx) {
        let frame = ctx.swap_chain.get_next_texture();
        let mut encoder = ctx.device.create_command_encoder(
            &wgpu::CommandEncoderDescriptor { todo: 0 },
        );

        // reset the indirect command buffer
        {
            let indirect_command = DrawIndirectCommand::default();
            let staging_buffer = ctx
                .device
                .create_buffer_mapped(1, wgpu::BufferUsage::COPY_SRC)
                .fill_from_slice(&[indirect_command]);
            encoder.copy_buffer_to_buffer(
                &staging_buffer,
                0,
                &self.draw_indirect_command,
                0,
                std::mem::size_of::<DrawIndirectCommand>()
                    as wgpu::BufferAddress,
            );
        }

        {
            let mut compute_pass = encoder.begin_compute_pass();
            compute_pass.set_pipeline(&self.compute_pipeline);
            compute_pass.set_bind_group(
                0,
                &self.compute_uniforms_bind_group,
                &[],
            );
            compute_pass.set_bind_group(1, &self.compute_src_bind_group, &[]);
            compute_pass.set_bind_group(2, &self.compute_dst_bind_group, &[]);
            compute_pass.dispatch(self.mesh.simplex_count, 1, 1);
        }

        {
            let mut render_pass =
                encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    color_attachments: &[
                        wgpu::RenderPassColorAttachmentDescriptor {
                            attachment: &frame.view,
                            resolve_target: None,
                            load_op: wgpu::LoadOp::Clear,
                            store_op: wgpu::StoreOp::Store,
                            clear_color: wgpu::Color {
                                r: 0.0,
                                g: 0.0,
                                b: 0.0,
                                a: 1.0,
                            },
                        },
                    ],
                    depth_stencil_attachment: None,
                });

            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_vertex_buffers(0, &[(&self.dst_vertices, 0)]);
            render_pass.draw_indirect(&self.draw_indirect_command, 0);
        }

        ctx.queue.submit(&[encoder.finish()]);

        /*
        self.draw_indirect_command.map_read_async(
            0,
            std::mem::size_of::<DrawIndirectCommand>() as wgpu::BufferAddress,
            |result: wgpu::BufferMapAsyncResult<&[DrawIndirectCommand]>| {
                if let Ok(e) = result {
                    println!("{:?}", e.data);
                }
            },
        );

        self.dst_vertices.map_read_async(
            0,
            288 * std::mem::size_of::<mesh4::Vertex4>() as wgpu::BufferAddress,
            |result: wgpu::BufferMapAsyncResult<&[mesh4::Vertex4]>| {
                if let Ok(e) = result {
                    println!("{:?}", e.data);
                }
            },
        );
        */
    }
}

fn main() -> Result<()> {
    context::run::<TestApp>("Hello world!", (800, 600))
}

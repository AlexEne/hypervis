#![no_std]
#![no_main]
#![feature(core_intrinsics, lang_items, link_args, alloc_error_handler)]

#[allow(unused_attributes)]
#[link_args = "/NODEFAULTLIB /SUBSYSTEM:CONSOLE /SAFESEH:NO /DYNAMICBASE:NO /ENTRY:mainCRTStartup /LTCG msvcrt.lib vcruntime.lib ucrt.lib msvcprt.lib oldnames.lib /FORCE"]
extern "C" {}
#[macro_use]
extern crate alloc;

use core::panic::PanicInfo;
use alloc::boxed::Box;
use core::alloc::{GlobalAlloc, Layout};
use core::hint::unreachable_unchecked;
use winapi::um::heapapi::{GetProcessHeap, HeapAlloc, HeapFree};
use winapi::um::processthreadsapi::ExitProcess;

mod alg;
mod context;
mod geometry;
mod mesh4;

use anyhow::Result;
use cgmath::{prelude::Zero, Matrix4, Vector4};
use winit::event::WindowEvent;

use context::graphics::{
    MeshBinding, SlicePipeline, SlicePlane, Transform4, TriangleListPipeline,
    ViewProjection, DEPTH_FORMAT,
};
use context::{Application, Ctx};

struct TestApp {
    render_pipeline: TriangleListPipeline,
    slice_pipeline: SlicePipeline,
    mesh_binding: MeshBinding,
    slice_plane: SlicePlane,
    depth_texture: wgpu::Texture,
    depth_texture_view: wgpu::TextureView,
    view_proj: ViewProjection,
    angular_vel: alg::Bivec4,
    rotor: alg::Rotor4,
    frames: usize,
}

impl Application for TestApp {
    fn init(ctx: &mut Ctx) -> Self {
        let _diagonal = SlicePlane {
            normal: Vector4::new(0.5, 0.5, 0.5, 0.5),
            base_point: Vector4::zero(),
            #[rustfmt::skip]
            proj_matrix: Matrix4::new(
                0.5, 0.5, 0.5, 0.0,
                0.5, -0.5, -0.5, 0.0,
                -0.5, 0.5, -0.5, 0.0,
                -0.5, -0.5, 0.5, 0.0,
            ),
        };

        let orthogonal = SlicePlane {
            normal: Vector4::new(0.0, 0.0, 0.0, 1.0),
            base_point: Vector4::new(0.0, 0.0, 0.0, 0.5),
            #[rustfmt::skip]
            proj_matrix: Matrix4::new(
                1.0, 0.0, 0.0, 0.0,
                0.0, 1.0, 0.0, 0.0,
                0.0, 0.0, 1.0, 0.0,
                0.0, 0.0, 0.0, 0.0,
            ),
        };

        let slice_plane = orthogonal;

        let rotor = alg::Rotor4::identity();
        let angular_vel = alg::Bivec4::new(0.0, 0.0, 0.1, 0.0, 0.1, 0.1);

        let view_proj = ViewProjection::new(ctx);

        let render_pipeline =
            TriangleListPipeline::new(&ctx.graphics_ctx).unwrap();
        let slice_pipeline = SlicePipeline::new(&ctx.graphics_ctx).unwrap();

        let mesh = mesh4::cell_120();
        let mesh_binding = slice_pipeline.create_mesh_binding(
            &ctx.graphics_ctx,
            &mesh.vertices,
            &mesh.indices,
        );

        let depth_texture =
            ctx.graphics_ctx
                .device
                .create_texture(&wgpu::TextureDescriptor {
                    format: DEPTH_FORMAT,
                    usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
                    ..ctx.graphics_ctx.sc_desc.to_texture_desc()
                });
        let depth_texture_view = depth_texture.create_default_view();

        TestApp {
            render_pipeline,
            slice_pipeline,
            slice_plane,
            mesh_binding,
            depth_texture,
            depth_texture_view,
            view_proj,
            rotor,
            angular_vel,
            frames: 0,
        }
    }

    fn resize(&mut self, ctx: &mut Ctx) {
        // update the projection
        self.view_proj = ViewProjection::new(ctx);

        self.depth_texture =
            ctx.graphics_ctx
                .device
                .create_texture(&wgpu::TextureDescriptor {
                    format: DEPTH_FORMAT,
                    usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
                    ..ctx.graphics_ctx.sc_desc.to_texture_desc()
                });
        self.depth_texture_view = self.depth_texture.create_default_view();
    }

    fn on_event(&mut self, _ctx: &mut Ctx, event: WindowEvent) {
        match event {
            _ => (),
        }
    }

    fn update(&mut self, _ctx: &mut Ctx) {
        // Update the slice
        let scale = (self.frames % 600) as f32 / 600.0 * 2.0 - 1.0;
        self.slice_plane.base_point = Vector4::new(0.0, 0.0, 0.0, scale);

        // Update the rotation
        // println!("{}", self.frames);
        let dt = 1f32 / 60f32;
        self.rotor.update(&(dt * self.angular_vel.clone()));
        // println!("{:?}", self.rotor);
    }

    fn render(&mut self, ctx: &mut Ctx) {
        let mut encoder = ctx.graphics_ctx.device.create_command_encoder(
            &wgpu::CommandEncoderDescriptor { todo: 0 },
        );

        let transform = Transform4 {
            displacement: Vector4::zero(),
            transform: self.rotor.to_matrix(),
        };
        self.slice_pipeline.render_mesh(
            &ctx.graphics_ctx,
            &mut encoder,
            &self.slice_plane,
            &transform,
            &self.mesh_binding,
        );

        // for some reason I need to do the compute and render passes in two
        // goes to have it work on vulkan without visual glitches
        ctx.graphics_ctx.queue.submit(&[encoder.finish()]);

        self.render_pipeline
            .update_view_proj(&mut ctx.graphics_ctx, &self.view_proj);

        let mut encoder = ctx.graphics_ctx.device.create_command_encoder(
            &wgpu::CommandEncoderDescriptor { todo: 0 },
        );

        let frame = ctx.graphics_ctx.swap_chain.get_next_texture();
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
                    depth_stencil_attachment: Some(
                        wgpu::RenderPassDepthStencilAttachmentDescriptor {
                            attachment: &self.depth_texture_view,
                            depth_load_op: wgpu::LoadOp::Clear,
                            depth_store_op: wgpu::StoreOp::Store,
                            clear_depth: 1.0,
                            stencil_load_op: wgpu::LoadOp::Clear,
                            stencil_store_op: wgpu::StoreOp::Store,
                            clear_stencil: 0,
                        },
                    ),
                });

            self.render_pipeline.render(
                &mut render_pass,
                self.slice_pipeline.indirect_command_buffer(),
                self.slice_pipeline.dst_vertex_buffer(),
            );
        }

        ctx.graphics_ctx.queue.submit(&[encoder.finish()]);

        self.frames += 1;
    }
}

fn run_app() {
    context::run::<TestApp>("Hello world!", (800, 600)).unwrap();
}

fn exit_process(exit_code: i32) -> ! {
    unsafe {
        ExitProcess(exit_code as _);
        unreachable_unchecked()
    }
}


struct SystemAllocator;
unsafe impl GlobalAlloc for SystemAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        HeapAlloc(GetProcessHeap(), 0, layout.size()) as *mut _
    }
    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        // tbh in a 64k or lower you could probably get away with just leaking memory here
        HeapFree(GetProcessHeap(), 0, ptr as *mut _);
    }
}

#[global_allocator]
static A: SystemAllocator = SystemAllocator;

// #[panic_handler]
// #[no_mangle]
// pub extern "C" fn panic(_info: &PanicInfo) -> ! {
//     // A small trick that can save some size - tell the optimizer that this point is unreachable,
//     // thereby allowing to to assume branches that panic never actually happen, which they
//     // shouldn't in a release build. Obviously very unsafe, so you probably don't want this in
//     // debug builds.
//     unsafe {
//         unreachable_unchecked()
//     }
// }

// #[alloc_error_handler]
// fn error_handler(_: core::alloc::Layout) -> ! {
//     // Same as above
//     unsafe {
//         unreachable_unchecked()
//     }
// }

// #[lang = "eh_personality"]
// extern "C" fn eh_personality() {}

#[no_mangle]
pub unsafe extern "C" fn mainCRTStartup() -> i32 {
    // Do some stuff
    let a = Box::new(10);
    
    run_app();
    
    // Note: returning from this function actually end the process, we need to call ExitProcess explicitly:
    exit_process(*a);
}

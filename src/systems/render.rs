use std::iter;
use bevy_ecs::prelude::{Mut, NonSend, Query};
use crate::components::{Camera, RenderModel, Skybox};
use crate::device::Device;

pub fn render_frame(
    mut q_skybox: Query<&mut Skybox>,
    mut q_render_models: Query<&mut RenderModel>,
    q_camera: Query<&Camera>,
    device: NonSend<Device>,
) {
    let surface_tex = device
        .surface
        .get_current_texture()
        .expect("Missing surface texture");
    let surface_tex_view = surface_tex.texture.create_view(&wgpu::TextureViewDescriptor::default());
    let color_attachment = Some(wgpu::RenderPassColorAttachment {
        view: &surface_tex_view,
        resolve_target: None,
        ops: wgpu::Operations {
            load: wgpu::LoadOp::Clear(wgpu::Color::RED),
            store: true,
        },
    });

    let depth_tex_view = device.depth_tex.as_ref().unwrap().view();
    let depth_attachment = Some(wgpu::RenderPassDepthStencilAttachment {
        view: depth_tex_view,
        depth_ops: Some(wgpu::Operations {
            load: wgpu::LoadOp::Clear(1.0),
            store: true,
        }),
        stencil_ops: None,
    });

    let camera = q_camera.iter().next().unwrap();
    // Couldn't make it work with using a single bundler encoder due to lifetimes
    let render_bundles = q_render_models.iter_mut()
        .map(|mut r| {
            let mut bundle_encoder =
                device.device
                    .create_render_bundle_encoder(&wgpu::RenderBundleEncoderDescriptor {
                        label: None,
                        multiview: None,
                        sample_count: 1,
                        color_formats: &[Some(device.surface_texture_format())],
                        depth_stencil: Some(wgpu::RenderBundleDepthStencil {
                            format: device.depth_texture_format(),
                            depth_read_only: false,
                            stencil_read_only: false,
                        }),
                    });
            r.render(&device, &camera, &mut bundle_encoder);
            bundle_encoder.finish(&wgpu::RenderBundleDescriptor { label: None })
        })
        .collect::<Vec<_>>();

    // let mut skybox = q_skybox.iter_mut().next().unwrap();
    // skybox.render(&device, &camera, &mut bundle_encoder);

    let cmd_buffer = {
        let mut encoder = device.device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[color_attachment],
                depth_stencil_attachment: depth_attachment,
            });
            pass.execute_bundles(render_bundles.iter().map(|b| b));
        }
        encoder.finish()
    };

    device.queue.submit(Some(cmd_buffer));

    surface_tex.present();
}
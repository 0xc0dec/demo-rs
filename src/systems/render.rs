use bevy_ecs::prelude::{NonSend, Query};
use crate::components::{Camera, Skybox};
use crate::device::Device;

pub fn render_frame(
    mut q_skybox: Query<&mut Skybox>,
    q_camera: Query<&Camera>,
    device: NonSend<Device>
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

    let cmd_buffer = {
        let mut encoder = device.device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        {
            let mut skybox = q_skybox.iter_mut().next().unwrap();
            let camera = q_camera.iter().next().unwrap();

            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[color_attachment],
                depth_stencil_attachment: depth_attachment,
            });

            skybox.render(&device, camera, &mut pass);
        }

        encoder.finish()
    };

    device.queue.submit(Some(cmd_buffer));

    surface_tex.present();
}
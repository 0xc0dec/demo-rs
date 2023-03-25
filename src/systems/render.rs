use bevy_ecs::prelude::{NonSend, NonSendMut, Query};
use winit::window::Window;
use crate::components::{Camera, RenderLayer, ModelRenderer};
use crate::debug_ui::DebugUI;
use crate::device::Device;

fn new_bundle_encoder(device: &Device) -> wgpu::RenderBundleEncoder {
    device.device.create_render_bundle_encoder(&wgpu::RenderBundleEncoderDescriptor {
        label: None,
        multiview: None,
        sample_count: 1,
        color_formats: &[Some(device.surface_texture_format())],
        depth_stencil: Some(wgpu::RenderBundleDepthStencil {
            format: device.depth_texture_format(),
            depth_read_only: false,
            stencil_read_only: false,
        }),
    })
}

pub fn render_frame(
    mut model_renderers: Query<(&mut ModelRenderer, &RenderLayer)>,
    mut debug_ui: NonSendMut<DebugUI>,
    cameras: Query<&Camera>,
    device: NonSend<Device>,
    window: NonSend<Window>
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

    let camera = cameras.single();
    // Couldn't make it work with a single bundler encoder due to lifetimes
    let mut render_bundles = model_renderers
        .iter_mut()
        .map(|(mut r, layer)| {
            let mut encoder = new_bundle_encoder(&device);
            // TODO Create render bundle inside the function?
            r.render(&device, &camera, &mut encoder);
            let bundle = encoder.finish(&wgpu::RenderBundleDescriptor { label: None });
            (bundle, layer.0)
        })
        .collect::<Vec<_>>();
    render_bundles.sort_by_key(|(_, layer)| *layer);

    let cmd_buffer = {
        let mut encoder = device.device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[color_attachment],
                depth_stencil_attachment: depth_attachment,
            });

            pass.execute_bundles(render_bundles.iter().map(|(bundle, _)| bundle));

            debug_ui.build_frame(&window, |_| {});
            debug_ui.render(&device, &mut pass);
        }
        encoder.finish()
    };

    device.queue.submit(Some(cmd_buffer));
    surface_tex.present();
}
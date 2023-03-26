use bevy_ecs::prelude::{NonSend, NonSendMut, Query};
use wgpu::RenderBundle;
use crate::components::{Camera, RenderOrder, ModelRenderer, Transform};
use crate::debug_ui::DebugUI;
use crate::device::Device;
use crate::render_target::RenderTarget;

fn render(
    device: &Device,
    bundles: &[(RenderBundle, i32)],
    target: Option<&RenderTarget>,
    debug_ui: &mut DebugUI
) {
    let surface_tex = target.is_none().then(|| {
        device
            .surface
            .get_current_texture()
            .expect("Missing surface texture")
    });
    let surface_tex_view = surface_tex.as_ref().map(|t| {
        t.texture.create_view(&wgpu::TextureViewDescriptor::default())
    });

    let color_tex_view = target
        .map(|t| t.color_tex().view())
        .or(surface_tex_view.as_ref())
        .unwrap();
    let color_attachment = Some(wgpu::RenderPassColorAttachment {
        view: color_tex_view,
        resolve_target: None,
        ops: wgpu::Operations {
            load: wgpu::LoadOp::Clear(wgpu::Color::RED),
            store: true,
        },
    });

    let depth_tex_view = target
        .map(|t| t.depth_tex().view())
        .or(device.depth_tex.as_ref().map(|t| t.view()))
        .unwrap();
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
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[color_attachment],
                depth_stencil_attachment: depth_attachment,
            });

            pass.execute_bundles(bundles.iter().map(|(bundle, _)| bundle));
            // TODO Make it optional because we only want to render it for the final on-screen frame
            debug_ui.render(&device, &mut pass);
        }

        encoder.finish()
    };

    device.queue.submit(Some(cmd_buffer));
    surface_tex.map(|t| t.present());
}

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

fn build_render_bundles(
    mut renderers: Query<(&mut ModelRenderer, Option<&RenderOrder>, &Transform)>,
    camera: (&Camera, &Transform),
    device: &Device
) -> Vec<(RenderBundle, i32)> {
    // Couldn't make it work with a single bundler encoder due to lifetimes
    let mut bundles = renderers
        .iter_mut()
        .map(|(mut r, order, tr)| {
            let mut encoder = new_bundle_encoder(&device);
            // TODO Create render bundle inside the function?
            r.render(&device, camera, &tr, &mut encoder);
            let bundle = encoder.finish(&wgpu::RenderBundleDescriptor { label: None });
            (bundle, order.map_or(0, |o| o.0))
        })
        .collect::<Vec<_>>();
    bundles.sort_by_key(|(_, order)| *order);
    bundles
}

pub fn render_frame(
    renderers: Query<(&mut ModelRenderer, Option<&RenderOrder>, &Transform)>,
    cameras: Query<(&Camera, &Transform)>,
    device: NonSend<Device>,
    mut debug_ui: NonSendMut<DebugUI>,
) {
    // TODO Take all cameras and render for each of them.
    // TODO First render cameras with render targets, then without.
    // TODO Each camera can have a layer filter, render only objects that satisfy it
    // TODO (e.g. layer "scene" and layer "post-process")
    // TODO The current render layers should be renamed to render order.
    let camera = cameras.single();
    let bundles = build_render_bundles(renderers, camera, &device);
    render(&device, &bundles, None, &mut debug_ui);
}
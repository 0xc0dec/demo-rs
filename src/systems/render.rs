use crate::assets::DrawMesh;
use bevy_ecs::prelude::*;
use wgpu::RenderBundle;

use crate::components::{Camera, Material, Mesh, RenderOrder, RenderTags, Transform};
use crate::debug_ui::DebugUI;
use crate::render_tags::RENDER_TAG_DEBUG_UI;
use crate::render_target::RenderTarget;
use crate::resources::Device;

fn render_pass(
    device: &Device,
    bundles: &[RenderBundle],
    target: Option<&RenderTarget>,
    debug_ui: Option<&mut DebugUI>,
) {
    let surface_tex = target.is_none().then(|| {
        device
            .surface()
            .get_current_texture()
            .expect("Missing surface texture")
    });
    let surface_tex_view = surface_tex.as_ref().map(|t| {
        t.texture
            .create_view(&wgpu::TextureViewDescriptor::default())
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
        .unwrap_or(device.depth_tex().view());
    let depth_attachment = Some(wgpu::RenderPassDepthStencilAttachment {
        view: depth_tex_view,
        depth_ops: Some(wgpu::Operations {
            load: wgpu::LoadOp::Clear(1.0),
            store: true,
        }),
        stencil_ops: None,
    });

    let cmd_buffer = {
        let mut encoder =
            device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[color_attachment],
                depth_stencil_attachment: depth_attachment,
            });

            pass.execute_bundles(bundles.iter());
            if let Some(ui) = debug_ui {
                ui.render(device, &mut pass)
            }
        }

        encoder.finish()
    };

    device.queue().submit(Some(cmd_buffer));
    if let Some(t) = surface_tex {
        t.present()
    }
}

fn new_bundle_encoder<'a>(
    device: &'a Device,
    target: Option<&RenderTarget>,
) -> wgpu::RenderBundleEncoder<'a> {
    let color_format = target.map_or(device.surface_texture_format(), |t| t.color_tex().format());
    let depth_format = target.map_or(device.depth_texture_format(), |t| t.depth_tex().format());

    device.create_render_bundle_encoder(&wgpu::RenderBundleEncoderDescriptor {
        label: None,
        multiview: None,
        sample_count: 1,
        color_formats: &[Some(color_format)],
        depth_stencil: Some(wgpu::RenderBundleDepthStencil {
            format: depth_format,
            depth_read_only: false,
            stencil_read_only: false,
        }),
    })
}

fn build_render_bundles<'a>(
    meshes: &mut [(
        &'a Mesh,
        &'a mut Material,
        &'a Transform,
        Option<&'a RenderTags>,
    )],
    camera: (&Camera, &Transform),
    device: &Device,
) -> Vec<RenderBundle> {
    // Couldn't make it work with a single bundler encoder due to lifetimes
    meshes
        .iter_mut()
        .filter(|(.., tags)| camera.0.should_render(tags.map_or(0u32, |t| t.0)))
        .map(|(mesh, ref mut mat, tr, _)| {
            let mut encoder = new_bundle_encoder(device, camera.0.target().as_ref());
            mat.apply(device, camera, tr, &mut encoder);
            encoder.draw_mesh(&mesh.0);
            encoder.finish(&wgpu::RenderBundleDescriptor { label: None })
        })
        .collect::<Vec<_>>()
}

pub fn render(
    cameras: Query<(&Camera, &Transform, Option<&RenderOrder>)>,
    mut meshes: Query<(
        &Mesh,
        &mut Material,
        &Transform,
        Option<&RenderOrder>,
        Option<&RenderTags>,
    )>,
    device: Res<Device>,
    mut debug_ui: NonSendMut<DebugUI>,
) {
    let mut cameras = cameras.into_iter().collect::<Vec<_>>();
    cameras.sort_by_key(|(.., order)| order.map_or(0, |o| o.0));
    let cameras = cameras.iter().map(|(c, t, ..)| (*c, *t));

    let mut meshes = meshes
        .iter_mut()
        .map(|(mesh, mat, tr, order, tags)| (mesh, mat.into_inner(), tr, order, tags))
        .collect::<Vec<_>>();
    meshes.sort_by_key(|(.., order, _)| order.map_or(0, |o| o.0));
    let mut meshes = meshes
        .into_iter()
        .map(|(mesh, mat, tr, _, tags)| (mesh, mat, tr, tags))
        .collect::<Vec<_>>();

    for camera in cameras {
        let bundles = build_render_bundles(&mut meshes, camera, &device);
        render_pass(
            &device,
            &bundles,
            camera.0.target().as_ref(),
            camera
                .0
                .should_render(RENDER_TAG_DEBUG_UI)
                .then(|| debug_ui.as_mut()),
        );
    }
}

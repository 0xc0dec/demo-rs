use wgpu::RenderBundle;

use crate::camera::Camera;
use crate::debug_ui::DebugUI;
use crate::graphics::Graphics;
use crate::materials::Material;
use crate::mesh::{DrawMesh, Mesh};
use crate::render_target::RenderTarget;
use crate::transform::Transform;

pub fn render_pass(
    gfx: &Graphics,
    bundles: &[RenderBundle],
    target: Option<&RenderTarget>,
    debug_ui: Option<&mut DebugUI>,
) {
    let surface_tex = target.is_none().then(|| {
        gfx.surface()
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
        .unwrap_or(gfx.depth_tex().view());
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
            gfx.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[color_attachment],
                depth_stencil_attachment: depth_attachment,
            });

            pass.execute_bundles(bundles.iter());
            if let Some(ui) = debug_ui {
                ui.render(gfx, &mut pass)
            }
        }

        encoder.finish()
    };

    gfx.queue().submit(Some(cmd_buffer));
    if let Some(t) = surface_tex {
        t.present()
    }
}

fn new_bundle_encoder<'a>(
    gfx: &'a Graphics,
    target: Option<&RenderTarget>,
) -> wgpu::RenderBundleEncoder<'a> {
    let color_format = target.map_or(gfx.surface_texture_format(), |t| t.color_tex().format());
    let depth_format = target.map_or(gfx.depth_texture_format(), |t| t.depth_tex().format());

    gfx.create_render_bundle_encoder(&wgpu::RenderBundleEncoderDescriptor {
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

pub fn build_render_bundle(
    mesh: &Mesh,
    material: &mut dyn Material,
    transform: &Transform,
    camera: (&Camera, &Transform),
    gfx: &Graphics,
) -> RenderBundle {
    let mut encoder = new_bundle_encoder(gfx, camera.0.target().as_ref());
    material.apply(&mut encoder, gfx, camera, transform);
    encoder.draw_mesh(mesh);
    encoder.finish(&wgpu::RenderBundleDescriptor { label: None })
}

use crate::render::PosTexCoordNormalVertex;
use crate::render::Texture;
use crate::render::{RenderPipelineParams, Renderer};

use super::super::components::{Camera, Transform};
use super::super::Assets;
use super::uniforms::ViewInvProjUniform;

pub struct SkyboxMaterial {
    pipeline: wgpu::RenderPipeline,
    tex_bind_group: wgpu::BindGroup,
    uniform_buf: wgpu::Buffer,
    uniform_bind_group: wgpu::BindGroup,
}

impl SkyboxMaterial {
    pub fn new(rr: &Renderer, assets: &Assets, texture: &Texture) -> Self {
        let (uniform_bind_group_layout, uniform_bind_group, uniform_buf) =
            rr.new_uniform_bind_group(bytemuck::cast_slice(&[ViewInvProjUniform::default()]));

        let (tex_bind_group_layout, tex_bind_group) =
            rr.new_texture_bind_group(texture, wgpu::TextureViewDimension::Cube);

        let pipeline = rr.new_render_pipeline(RenderPipelineParams {
            shader_module: assets.shader(assets.skybox_shader),
            depth_write: false,
            depth_enabled: true,
            bind_group_layouts: &[&uniform_bind_group_layout, &tex_bind_group_layout],
            vertex_buffer_layouts: &[PosTexCoordNormalVertex::buffer_layout()],
        });

        Self {
            pipeline,
            tex_bind_group,
            uniform_buf,
            uniform_bind_group,
        }
    }
}

impl SkyboxMaterial {
    pub fn set_wvp(&self, rr: &Renderer, cam: &Camera, cam_tr: &Transform) {
        rr.queue().write_buffer(
            &self.uniform_buf,
            0,
            bytemuck::cast_slice(&[ViewInvProjUniform::new(
                &cam_tr.view_matrix(),
                &cam.proj_matrix(),
            )]),
        );
    }

    pub fn apply<'a>(&'a self, encoder: &mut wgpu::RenderBundleEncoder<'a>) {
        encoder.set_pipeline(&self.pipeline);
        encoder.set_bind_group(0, &self.uniform_bind_group, &[]);
        encoder.set_bind_group(1, &self.tex_bind_group, &[]);
    }
}

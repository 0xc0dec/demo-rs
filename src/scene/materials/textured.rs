use crate::render::PosTexCoordNormalVertex;
use crate::render::Texture;
use crate::render::{RenderPipelineParams, Renderer};

use super::super::components::{Camera, Transform};
use super::super::Assets;
use super::uniforms::WorldViewProjUniform;

pub struct TexturedMaterial {
    pipeline: wgpu::RenderPipeline,
    texture_bind_group: wgpu::BindGroup,
    uniform_buf: wgpu::Buffer,
    uniform_bind_group: wgpu::BindGroup,
}

impl TexturedMaterial {
    pub fn new(rr: &Renderer, assets: &Assets, texture: &Texture) -> Self {
        let (uniform_bind_group_layout, uniform_bind_group, uniform_buf) =
            rr.new_uniform_bind_group(bytemuck::cast_slice(&[WorldViewProjUniform::default()]));

        let (texture_bind_group_layout, texture_bind_group) =
            rr.new_texture_bind_group(texture, wgpu::TextureViewDimension::D2);

        let pipeline = rr.new_render_pipeline(RenderPipelineParams {
            shader_module: assets.shader(assets.textured_shader),
            depth_write: true,
            depth_enabled: true,
            wireframe: false,
            bind_group_layouts: &[&texture_bind_group_layout, &uniform_bind_group_layout],
            vertex_buffer_layouts: &[PosTexCoordNormalVertex::buffer_layout()],
        });

        Self {
            texture_bind_group,
            uniform_buf,
            uniform_bind_group,
            pipeline,
        }
    }
}

impl TexturedMaterial {
    pub fn set_wvp(&self, rr: &Renderer, cam: &Camera, cam_tr: &Transform, tr: &Transform) {
        rr.queue().write_buffer(
            &self.uniform_buf,
            0,
            bytemuck::cast_slice(&[WorldViewProjUniform::new(
                &tr.matrix(),
                &cam_tr.view_matrix(),
                &cam.proj_matrix(),
            )]),
        );
    }

    pub fn apply<'a>(&'a self, encoder: &mut wgpu::RenderBundleEncoder<'a>) {
        encoder.set_pipeline(&self.pipeline);
        encoder.set_bind_group(0, &self.texture_bind_group, &[]);
        encoder.set_bind_group(1, &self.uniform_bind_group, &[]);
    }
}

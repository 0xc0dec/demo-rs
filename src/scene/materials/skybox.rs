use crate::render::{ApplyMaterial, PosTexCoordNormalVertex};
use crate::render::Texture;
use crate::render::{RenderPipelineParams, Renderer};
use crate::scene::components::{Camera, Transform};
use crate::scene::Assets;

use super::uniforms::ViewInvProjUniform;

pub struct SkyboxMaterial {
    pipeline: wgpu::RenderPipeline,
    texture_bind_group: wgpu::BindGroup,
    matrices_uniform: ViewInvProjUniform,
    matrices_uniform_buf: wgpu::Buffer,
    matrices_uniform_bind_group: wgpu::BindGroup,
}

impl SkyboxMaterial {
    pub fn new(rr: &Renderer, assets: &Assets, texture: &Texture) -> Self {
        let matrices_uniform = ViewInvProjUniform::default();
        let (matrices_uniform_bind_group_layout, matrices_uniform_bind_group, matrices_uniform_buf) =
            rr.new_uniform_bind_group(bytemuck::cast_slice(&[matrices_uniform]));

        let (texture_bind_group_layout, texture_bind_group) =
            rr.new_texture_bind_group(texture, wgpu::TextureViewDimension::Cube);

        let pipeline = rr.new_render_pipeline(RenderPipelineParams {
            shader_module: assets.shader(assets.skybox_shader),
            depth_write: false,
            depth_enabled: true,
            bind_group_layouts: &[
                &matrices_uniform_bind_group_layout,
                &texture_bind_group_layout,
            ],
            vertex_buffer_layouts: &[PosTexCoordNormalVertex::buffer_layout()],
        });

        Self {
            pipeline,
            texture_bind_group,
            matrices_uniform,
            matrices_uniform_buf,
            matrices_uniform_bind_group,
        }
    }
}

impl SkyboxMaterial {
    pub fn set_wvp(&mut self, rr: &Renderer, camera: &Camera, camera_transform: &Transform) {
        self.matrices_uniform
            .update(&camera_transform.view_matrix(), &camera.proj_matrix());
        rr.queue().write_buffer(
            &self.matrices_uniform_buf,
            0,
            bytemuck::cast_slice(&[self.matrices_uniform]),
        );
    }
}

impl ApplyMaterial for SkyboxMaterial {
    fn apply<'a>(&'a self, encoder: &mut wgpu::RenderBundleEncoder<'a>) {
        encoder.set_pipeline(&self.pipeline);
        encoder.set_bind_group(0, &self.matrices_uniform_bind_group, &[]);
        encoder.set_bind_group(1, &self.texture_bind_group, &[]);
    }
}

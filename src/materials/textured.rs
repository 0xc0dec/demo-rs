use crate::assets::Assets;
use crate::components::{Camera, Transform};
use crate::renderer::{RenderPipelineParams, Renderer};
use crate::texture::Texture;
use crate::vertex::PosTexCoordNormalVertex;

use super::apply_material::ApplyMaterial;
use super::uniforms::WorldViewProjUniform;

pub struct TexturedMaterial {
    pipeline: wgpu::RenderPipeline,
    texture_bind_group: wgpu::BindGroup,
    matrices_uniform: WorldViewProjUniform,
    matrices_uniform_buf: wgpu::Buffer,
    matrices_uniform_bind_group: wgpu::BindGroup,
}

impl TexturedMaterial {
    pub fn new(rr: &Renderer, assets: &Assets, texture: &Texture) -> Self {
        let matrices_uniform = WorldViewProjUniform::default();
        let (matrices_uniform_bind_group_layout, matrices_uniform_bind_group, matrices_uniform_buf) =
            rr.new_uniform_bind_group(bytemuck::cast_slice(&[matrices_uniform]));

        let (texture_bind_group_layout, texture_bind_group) =
            rr.new_texture_bind_group(texture, wgpu::TextureViewDimension::D2);

        let pipeline = rr.new_render_pipeline(RenderPipelineParams {
            shader_module: assets.shader(assets.textured_shader),
            depth_write: true,
            depth_enabled: true,
            bind_group_layouts: &[
                &texture_bind_group_layout,
                &matrices_uniform_bind_group_layout,
            ],
            vertex_buffer_layouts: &[PosTexCoordNormalVertex::buffer_layout()],
        });

        Self {
            texture_bind_group,
            matrices_uniform,
            matrices_uniform_buf,
            matrices_uniform_bind_group,
            pipeline,
        }
    }
}

impl TexturedMaterial {
    pub fn set_wvp(
        &mut self,
        rr: &Renderer,
        camera: &Camera,
        camera_transform: &Transform,
        transform: &Transform,
    ) {
        self.matrices_uniform.update(
            &transform.matrix(),
            &camera_transform.view_matrix(),
            &camera.proj_matrix(),
        );
        rr.queue().write_buffer(
            &self.matrices_uniform_buf,
            0,
            bytemuck::cast_slice(&[self.matrices_uniform]),
        );
    }
}

impl ApplyMaterial for TexturedMaterial {
    fn apply<'a>(&'a self, encoder: &mut wgpu::RenderBundleEncoder<'a>) {
        encoder.set_pipeline(&self.pipeline);
        encoder.set_bind_group(0, &self.texture_bind_group, &[]);
        encoder.set_bind_group(1, &self.matrices_uniform_bind_group, &[]);
    }
}

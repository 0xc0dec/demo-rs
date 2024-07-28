use crate::assets::Assets;
use crate::components::{Camera, Transform};
use crate::graphics::{Graphics, RenderPipelineParams};
use crate::mesh::MeshVertex;
use crate::texture::Texture;

use super::material::Material;
use super::uniforms::ViewInvProjUniform;

pub struct SkyboxMaterial {
    pipeline: wgpu::RenderPipeline,
    texture_bind_group: wgpu::BindGroup,
    matrices_uniform: ViewInvProjUniform,
    matrices_uniform_buf: wgpu::Buffer,
    matrices_uniform_bind_group: wgpu::BindGroup,
}

impl SkyboxMaterial {
    pub fn new(gfx: &Graphics, assets: &Assets, texture: &Texture) -> Self {
        let matrices_uniform = ViewInvProjUniform::new();
        let (matrices_uniform_bind_group_layout, matrices_uniform_bind_group, matrices_uniform_buf) =
            gfx.new_uniform_bind_group(bytemuck::cast_slice(&[matrices_uniform]));

        let (texture_bind_group_layout, texture_bind_group) =
            gfx.new_texture_bind_group(texture, wgpu::TextureViewDimension::Cube);

        let pipeline = gfx.new_render_pipeline(RenderPipelineParams {
            shader_module: assets.shader(assets.skybox_shader_handle),
            depth_write: false,
            depth_enabled: true,
            bind_group_layouts: &[
                &matrices_uniform_bind_group_layout,
                &texture_bind_group_layout,
            ],
            vertex_buffer_layouts: &[MeshVertex::buffer_layout()],
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

impl Material for SkyboxMaterial {
    fn apply<'a>(
        &'a mut self,
        encoder: &mut wgpu::RenderBundleEncoder<'a>,
        gfx: &Graphics,
        camera: (&Camera, &Transform),
        _transform: &Transform,
    ) {
        self.matrices_uniform
            .update(&camera.1.view_matrix(), &camera.0.proj_matrix());
        gfx.queue().write_buffer(
            &self.matrices_uniform_buf,
            0,
            bytemuck::cast_slice(&[self.matrices_uniform]),
        );

        encoder.set_pipeline(&self.pipeline);
        encoder.set_bind_group(0, &self.matrices_uniform_bind_group, &[]);
        encoder.set_bind_group(1, &self.texture_bind_group, &[]);
    }
}

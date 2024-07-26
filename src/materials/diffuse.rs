use crate::assets::Assets;
use crate::camera::Camera;
use crate::graphics::{Graphics, RenderPipelineParams};
use crate::mesh::MeshVertex;
use crate::texture::Texture;
use crate::transform::Transform;

use super::material::Material;
use super::uniforms::WorldViewProjUniform;

// TODO Rename to smth less generic
pub struct DiffuseMaterial {
    pipeline: wgpu::RenderPipeline,
    texture_bind_group: wgpu::BindGroup,
    matrices_uniform: WorldViewProjUniform,
    matrices_uniform_buf: wgpu::Buffer,
    matrices_uniform_bind_group: wgpu::BindGroup,
}

impl DiffuseMaterial {
    pub fn new(gfx: &Graphics, assets: &Assets, texture: &Texture) -> Self {
        let matrices_uniform = WorldViewProjUniform::new();
        let (matrices_uniform_bind_group_layout, matrices_uniform_bind_group, matrices_uniform_buf) =
            gfx.new_uniform_bind_group(bytemuck::cast_slice(&[matrices_uniform]));

        let (texture_bind_group_layout, texture_bind_group) =
            gfx.new_texture_bind_group(texture, wgpu::TextureViewDimension::D2);

        let pipeline = gfx.new_render_pipeline(RenderPipelineParams {
            shader_module: assets.diffuse_shader(),
            depth_write: true,
            depth_enabled: true,
            bind_group_layouts: &[
                &texture_bind_group_layout,
                &matrices_uniform_bind_group_layout,
            ],
            vertex_buffer_layouts: &[MeshVertex::buffer_layout()],
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

impl Material for DiffuseMaterial {
    fn apply<'a>(
        &'a mut self,
        encoder: &mut wgpu::RenderBundleEncoder<'a>,
        gfx: &Graphics,
        camera: (&Camera, &Transform),
        transform: &Transform,
    ) {
        self.matrices_uniform.update(
            &transform.matrix(),
            &camera.1.view_matrix(),
            &camera.0.proj_matrix(),
        );
        gfx.queue().write_buffer(
            &self.matrices_uniform_buf,
            0,
            bytemuck::cast_slice(&[self.matrices_uniform]),
        );

        encoder.set_pipeline(&self.pipeline);
        encoder.set_bind_group(0, &self.texture_bind_group, &[]);
        encoder.set_bind_group(1, &self.matrices_uniform_bind_group, &[]);
    }
}

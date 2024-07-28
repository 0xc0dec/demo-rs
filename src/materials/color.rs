use crate::assets::Assets;
use crate::components::{Camera, Transform};
use crate::graphics::{Graphics, RenderPipelineParams};
use crate::mesh::MeshVertex;

use super::material::Material;
use super::uniforms::WorldViewProjUniform;

pub struct ColorMaterial {
    pipeline: wgpu::RenderPipeline,
    matrices_uniform: WorldViewProjUniform,
    matrices_uniform_buf: wgpu::Buffer,
    matrices_uniform_bind_group: wgpu::BindGroup,
}

impl ColorMaterial {
    pub fn new(gfx: &Graphics, assets: &Assets) -> Self {
        let matrices_uniform = WorldViewProjUniform::new();
        let (matrices_uniform_bind_group_layout, matrices_uniform_bind_group, matrices_uniform_buf) =
            gfx.new_uniform_bind_group(bytemuck::cast_slice(&[matrices_uniform]));

        let pipeline = gfx.new_render_pipeline(RenderPipelineParams {
            shader_module: assets.shader(assets.color_shader_handle),
            depth_write: true,
            depth_enabled: true,
            bind_group_layouts: &[&matrices_uniform_bind_group_layout],
            vertex_buffer_layouts: &[MeshVertex::buffer_layout()],
        });

        Self {
            matrices_uniform,
            matrices_uniform_buf,
            matrices_uniform_bind_group,
            pipeline,
        }
    }
}

impl Material for ColorMaterial {
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
        encoder.set_bind_group(0, &self.matrices_uniform_bind_group, &[]);
    }
}

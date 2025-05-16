use crate::math::Vec3;
use crate::render::PosTexCoordNormalVertex;
use crate::render::{RenderPipelineParams, Renderer};
use crate::scene::components::{Camera, Transform};
use crate::scene::Assets;

use super::apply_material::ApplyMaterial;
use super::uniforms::{Vec3Uniform, WorldViewProjUniform};

pub struct ColorMaterial {
    pipeline: wgpu::RenderPipeline,
    matrices_uniform: WorldViewProjUniform,
    matrices_uniform_buf: wgpu::Buffer,
    matrices_uniform_bind_group: wgpu::BindGroup,
    color_uniform: Vec3Uniform,
    color_uniform_buf: wgpu::Buffer,
    color_uniform_bind_group: wgpu::BindGroup,
}

impl ColorMaterial {
    pub fn new(rr: &Renderer, assets: &Assets) -> Self {
        let matrices_uniform = WorldViewProjUniform::default();
        let (matrices_uniform_bind_group_layout, matrices_uniform_bind_group, matrices_uniform_buf) =
            rr.new_uniform_bind_group(bytemuck::cast_slice(&[matrices_uniform]));

        let color_uniform = Vec3Uniform::default();
        let (color_uniform_bind_group_layout, color_uniform_bind_group, color_uniform_buf) =
            rr.new_uniform_bind_group(bytemuck::cast_slice(&[color_uniform]));

        let pipeline = rr.new_render_pipeline(RenderPipelineParams {
            shader_module: assets.shader(assets.color_shader),
            depth_write: true,
            depth_enabled: true,
            bind_group_layouts: &[
                &matrices_uniform_bind_group_layout,
                &color_uniform_bind_group_layout,
            ],
            vertex_buffer_layouts: &[PosTexCoordNormalVertex::buffer_layout()],
        });

        Self {
            pipeline,
            matrices_uniform,
            matrices_uniform_buf,
            matrices_uniform_bind_group,
            color_uniform,
            color_uniform_buf,
            color_uniform_bind_group,
        }
    }
}

impl ColorMaterial {
    pub fn set_color(&mut self, rr: &Renderer, color: Vec3) {
        self.color_uniform.update(color);
        rr.queue().write_buffer(
            &self.color_uniform_buf,
            0,
            bytemuck::cast_slice(&[self.color_uniform]),
        );
    }

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

impl ApplyMaterial for ColorMaterial {
    fn apply<'a>(&'a self, encoder: &mut wgpu::RenderBundleEncoder<'a>) {
        encoder.set_pipeline(&self.pipeline);
        encoder.set_bind_group(0, &self.matrices_uniform_bind_group, &[]);
        encoder.set_bind_group(1, &self.color_uniform_bind_group, &[]);
    }
}

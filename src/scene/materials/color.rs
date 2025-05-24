use crate::math::Vec3;
use crate::render::PositionUvNormalVertex;
use crate::render::{RenderPipelineParams, Renderer};

use super::super::components::{Camera, Transform};
use super::uniforms::{Vec3Uniform, WorldViewProjUniform};

pub struct ColorMaterial {
    pipeline: wgpu::RenderPipeline,
    matrices_uniform_buf: wgpu::Buffer,
    matrices_uniform_bind_group: wgpu::BindGroup,
    color_uniform_bind_group: wgpu::BindGroup,
}

impl ColorMaterial {
    pub fn new(rr: &Renderer, shader: &wgpu::ShaderModule, color: Vec3, wireframe: bool) -> Self {
        let (matrices_uniform_bind_group_layout, matrices_uniform_bind_group, matrices_uniform_buf) =
            rr.new_uniform_bind_group(bytemuck::cast_slice(&[WorldViewProjUniform::default()]));

        let (color_uniform_bind_group_layout, color_uniform_bind_group, ..) =
            rr.new_uniform_bind_group(bytemuck::cast_slice(&[Vec3Uniform::new(color)]));

        let pipeline = rr.new_render_pipeline(RenderPipelineParams {
            shader_module: shader,
            depth_write: true,
            depth_enabled: true,
            wireframe,
            bind_group_layouts: &[
                &matrices_uniform_bind_group_layout,
                &color_uniform_bind_group_layout,
            ],
            // TODO Leaner vertex format. Can't use it currently because this material
            // is used for file-loaded meshes where we currently only support a single vertex format.
            vertex_buffer_layouts: &[PositionUvNormalVertex::buffer_layout()],
        });

        Self {
            pipeline,
            matrices_uniform_buf,
            matrices_uniform_bind_group,
            color_uniform_bind_group,
        }
    }
}

impl ColorMaterial {
    pub fn set_wvp(&self, rr: &Renderer, cam: &Camera, cam_tr: &Transform, tr: &Transform) {
        rr.queue().write_buffer(
            &self.matrices_uniform_buf,
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
        encoder.set_bind_group(0, &self.matrices_uniform_bind_group, &[]);
        encoder.set_bind_group(1, &self.color_uniform_bind_group, &[]);
    }
}

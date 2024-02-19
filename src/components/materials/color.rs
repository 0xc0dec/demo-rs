use wgpu::{BindGroup, RenderPipeline};

use super::utils::*;
use crate::assets::MeshVertex;
use crate::components::{Camera, Transform};
use crate::math::{Mat4, OPENGL_TO_WGPU_MATRIX};
use crate::resources::{Assets, Device};

pub struct ColorMaterial {
    pipeline: RenderPipeline,
    matrices_uniform: MatricesUniform,
    matrices_uniform_buf: wgpu::Buffer,
    matrices_uniform_bind_group: BindGroup,
}

impl ColorMaterial {
    pub fn new(device: &Device, assets: &Assets) -> Self {
        let matrices_uniform = MatricesUniform {
            world: Mat4::identity().into(),
            view_proj: Mat4::identity().into(),
        };
        let (matrices_uniform_bind_group_layout, matrices_uniform_bind_group, matrices_uniform_buf) =
            new_uniform_bind_group(device, bytemuck::cast_slice(&[matrices_uniform]));

        let pipeline = new_render_pipeline(
            device,
            RenderPipelineParams {
                shader_module: &assets.color_shader,
                depth_write: true,
                depth_enabled: true,
                bind_group_layouts: &[&matrices_uniform_bind_group_layout],
                vertex_buffer_layouts: &[MeshVertex::buffer_layout()],
            },
        );

        Self {
            matrices_uniform,
            matrices_uniform_buf,
            matrices_uniform_bind_group,
            pipeline,
        }
    }

    pub fn update_uniforms(
        &mut self,
        device: &Device,
        camera: (&Camera, &Transform),
        transform: &Transform,
    ) {
        self.matrices_uniform.world = transform.matrix().into();
        self.matrices_uniform.view_proj =
            (OPENGL_TO_WGPU_MATRIX * camera.0.proj_matrix() * camera.1.view_matrix()).into();
        device.queue().write_buffer(
            &self.matrices_uniform_buf,
            0,
            bytemuck::cast_slice(&[self.matrices_uniform]),
        );
    }

    pub fn apply<'a>(&'a mut self, encoder: &mut wgpu::RenderBundleEncoder<'a>) {
        encoder.set_pipeline(&self.pipeline);
        encoder.set_bind_group(0, &self.matrices_uniform_bind_group, &[]);
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct MatricesUniform {
    world: [[f32; 4]; 4],
    view_proj: [[f32; 4]; 4],
}

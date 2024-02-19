use super::utils::*;
use crate::assets::MeshVertex;
use crate::assets::Texture;
use crate::components::{Camera, Transform};
use crate::math::{Mat4, OPENGL_TO_WGPU_MATRIX};
use crate::resources::{Assets, Device};
use bevy_ecs::prelude::Component;

#[derive(Component)]
pub struct SkyboxMaterial {
    pipeline: wgpu::RenderPipeline,
    texture_bind_group: wgpu::BindGroup,
    matrices_uniform: MatricesUniform,
    matrices_uniform_buf: wgpu::Buffer,
    matrices_uniform_bind_group: wgpu::BindGroup,
}

impl SkyboxMaterial {
    pub fn new(device: &Device, assets: &Assets, texture: &Texture) -> Self {
        let matrices_uniform = MatricesUniform {
            view_mat: Mat4::identity().into(),
            proj_mat_inv: Mat4::identity().into(),
        };
        let (matrices_uniform_bind_group_layout, matrices_uniform_bind_group, matrices_uniform_buf) =
            new_uniform_bind_group(device, bytemuck::cast_slice(&[matrices_uniform]));

        let (texture_bind_group_layout, texture_bind_group) =
            new_texture_bind_group(device, texture, wgpu::TextureViewDimension::Cube);

        let pipeline = new_render_pipeline(
            device,
            RenderPipelineParams {
                shader_module: &assets.skybox_shader,
                depth_write: false,
                depth_enabled: true,
                bind_group_layouts: &[
                    &matrices_uniform_bind_group_layout,
                    &texture_bind_group_layout,
                ],
                vertex_buffer_layouts: &[MeshVertex::buffer_layout()],
            },
        );

        Self {
            pipeline,
            texture_bind_group,
            matrices_uniform,
            matrices_uniform_buf,
            matrices_uniform_bind_group,
        }
    }

    pub fn update_uniforms(&mut self, device: &Device, camera: (&Camera, &Transform)) {
        self.matrices_uniform.view_mat = camera.1.view_matrix().into();
        self.matrices_uniform.proj_mat_inv = (OPENGL_TO_WGPU_MATRIX * camera.0.proj_matrix())
            .try_inverse()
            .unwrap()
            .into();
        device.queue().write_buffer(
            &self.matrices_uniform_buf,
            0,
            bytemuck::cast_slice(&[self.matrices_uniform]),
        );
    }

    pub fn apply<'a>(&'a mut self, encoder: &mut wgpu::RenderBundleEncoder<'a>) {
        encoder.set_pipeline(&self.pipeline);
        encoder.set_bind_group(0, &self.matrices_uniform_bind_group, &[]);
        encoder.set_bind_group(1, &self.texture_bind_group, &[]);
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct MatricesUniform {
    // Couldn't make it work with Matrix3, probably something to do with alignment and padding
    view_mat: [[f32; 4]; 4],
    proj_mat_inv: [[f32; 4]; 4],
}

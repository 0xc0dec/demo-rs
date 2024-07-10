use super::apply_material::ApplyMaterial;
use super::utils::*;
use crate::assets::shaders::ViewInvProjUniform;
use crate::assets::{MeshVertex, Texture};
use crate::components::{Camera, Transform};
use crate::resources::{Assets, Device};

pub struct SkyboxMaterial {
    pipeline: wgpu::RenderPipeline,
    texture_bind_group: wgpu::BindGroup,
    matrices_uniform: ViewInvProjUniform,
    matrices_uniform_buf: wgpu::Buffer,
    matrices_uniform_bind_group: wgpu::BindGroup,
}

impl SkyboxMaterial {
    pub fn new(device: &Device, assets: &Assets, texture: &Texture) -> Self {
        let matrices_uniform = ViewInvProjUniform::new();
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
}

impl ApplyMaterial for SkyboxMaterial {
    fn apply<'a>(
        &'a mut self,
        encoder: &mut wgpu::RenderBundleEncoder<'a>,
        device: &Device,
        camera: (&Camera, &Transform),
        _transform: &Transform,
    ) {
        self.matrices_uniform
            .update(&camera.1.view_matrix(), &camera.0.proj_matrix());
        device.queue().write_buffer(
            &self.matrices_uniform_buf,
            0,
            bytemuck::cast_slice(&[self.matrices_uniform]),
        );

        encoder.set_pipeline(&self.pipeline);
        encoder.set_bind_group(0, &self.matrices_uniform_bind_group, &[]);
        encoder.set_bind_group(1, &self.texture_bind_group, &[]);
    }
}

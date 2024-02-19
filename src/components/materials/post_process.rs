use wgpu::{BindGroup, RenderPipeline};

use crate::assets::{MeshVertex, Texture};
use crate::components::{Camera, Transform};
use crate::resources::{Assets, Device};

use super::apply_material::ApplyMaterial;
use super::utils::*;

pub struct PostProcessMaterial {
    pipeline: RenderPipeline,
    texture_bind_group: BindGroup,
}

impl PostProcessMaterial {
    pub fn new(device: &Device, assets: &Assets, texture: &Texture) -> Self {
        let (texture_bind_group_layout, texture_bind_group) =
            new_texture_bind_group(device, texture, wgpu::TextureViewDimension::D2);

        let pipeline = new_render_pipeline(
            device,
            RenderPipelineParams {
                shader_module: &assets.postprocess_shader,
                depth_write: true,
                depth_enabled: true,
                bind_group_layouts: &[&texture_bind_group_layout],
                vertex_buffer_layouts: &[MeshVertex::buffer_layout()],
            },
        );

        Self {
            pipeline,
            texture_bind_group,
        }
    }
}

impl ApplyMaterial for PostProcessMaterial {
    fn apply<'a>(
        &'a mut self,
        encoder: &mut wgpu::RenderBundleEncoder<'a>,
        _device: &Device,
        _camera: (&Camera, &Transform),
        _transform: &Transform,
    ) {
        encoder.set_pipeline(&self.pipeline);
        encoder.set_bind_group(0, &self.texture_bind_group, &[]);
    }
}

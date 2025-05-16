use wgpu::{BindGroup, RenderPipeline};

use crate::assets::Assets;
use crate::render::Texture;
use crate::render::{RenderPipelineParams, Renderer};
use crate::vertex::PosTexCoordNormalVertex;

use super::apply_material::ApplyMaterial;

pub struct PostProcessMaterial {
    pipeline: RenderPipeline,
    texture_bind_group: BindGroup,
}

impl PostProcessMaterial {
    pub fn new(rr: &Renderer, assets: &Assets, texture: &Texture) -> Self {
        let (texture_bind_group_layout, texture_bind_group) =
            rr.new_texture_bind_group(texture, wgpu::TextureViewDimension::D2);

        let pipeline = rr.new_render_pipeline(RenderPipelineParams {
            shader_module: assets.shader(assets.postprocess_shader),
            depth_write: true,
            depth_enabled: true,
            bind_group_layouts: &[&texture_bind_group_layout],
            vertex_buffer_layouts: &[PosTexCoordNormalVertex::buffer_layout()],
        });

        Self {
            pipeline,
            texture_bind_group,
        }
    }
}

impl ApplyMaterial for PostProcessMaterial {
    fn apply<'a>(&'a self, encoder: &mut wgpu::RenderBundleEncoder<'a>) {
        encoder.set_pipeline(&self.pipeline);
        encoder.set_bind_group(0, &self.texture_bind_group, &[]);
    }
}

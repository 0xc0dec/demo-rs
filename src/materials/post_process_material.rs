use wgpu::{BindGroup, RenderPipeline};
use crate::model::{ModelVertex, Vertex};
use crate::graphics::Graphics;
use crate::materials::Material;
use crate::materials::utils::*;
use crate::texture::Texture;

pub struct PostProcessMaterial {
    pub texture: Texture,
    pipeline: RenderPipeline,
    texture_bind_group: BindGroup,
}

pub struct PostProcessMaterialParams {
    pub texture: Texture,
}

impl PostProcessMaterial {
    pub async fn new(gfx: &Graphics, params: PostProcessMaterialParams) -> Self {
        let (
            texture_bind_group_layout,
            texture_bind_group
        ) = new_texture_bind_group(gfx, &params.texture, wgpu::TextureViewDimension::D2);

        let pipeline = new_render_pipeline(
            gfx,
            RenderPipelineParams {
                shader_file_name: "post-process.wgsl",
                depth_write: true,
                depth_enabled: false,
                bind_group_layouts: &[
                    &texture_bind_group_layout
                ],
                vertex_buffer_layouts: &[ModelVertex::desc()]
            }
        ).await;

        Self {
            texture: params.texture,
            pipeline,
            texture_bind_group,
        }
    }
}

impl<'a, 'b> Material<'a, 'b> for PostProcessMaterial where 'a: 'b {
    fn apply(&'a mut self, pass: &mut wgpu::RenderPass<'b>) {
        pass.set_pipeline(&self.pipeline);
        pass.set_bind_group(0, &self.texture_bind_group, &[]);
    }
}

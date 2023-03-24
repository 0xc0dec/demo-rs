use crate::device::{Device};
use crate::model::{ModelVertex, Vertex};
use crate::shaders::utils::*;
use crate::shaders::Shader;
use crate::texture::Texture;
use wgpu::{BindGroup, RenderPipeline};

pub struct PostProcessShader {
    pipeline: RenderPipeline,
    texture_bind_group: BindGroup,
}

pub struct PostProcessShaderParams<'a> {
    pub texture: &'a Texture,
}

impl PostProcessShader {
    pub async fn new(device: &Device, params: PostProcessShaderParams<'_>) -> Self {
        let (texture_bind_group_layout, texture_bind_group) =
            new_texture_bind_group(device, &params.texture, wgpu::TextureViewDimension::D2);

        let pipeline = new_render_pipeline(
            device,
            RenderPipelineParams {
                shader_file_name: "post-process.wgsl",
                depth_write: true,
                depth_enabled: true,
                bind_group_layouts: &[&texture_bind_group_layout],
                vertex_buffer_layouts: &[ModelVertex::desc()],
            },
        )
        .await;

        Self {
            pipeline,
            texture_bind_group,
        }
    }
}

impl Shader for PostProcessShader {
    fn apply<'a, 'b>(&'a mut self, pass: &mut wgpu::RenderPass<'b>) where 'a: 'b {
        pass.set_pipeline(&self.pipeline);
        pass.set_bind_group(0, &self.texture_bind_group, &[]);
    }
}

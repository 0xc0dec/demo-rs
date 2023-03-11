use crate::device::{Device, Frame};
use crate::model::{DrawModel, Mesh};
use crate::render_target::RenderTarget;
use crate::shaders::{PostProcessShader, PostProcessShaderParams, Shader};
use crate::texture::TextureSize;

pub struct PostProcessor {
    rt: RenderTarget,
    shader: PostProcessShader,
    mesh: Mesh
}

impl PostProcessor {
    pub fn source_rt(&self) -> &RenderTarget {
        &self.rt
    }

    pub async fn new(device: &Device, size: TextureSize) -> Self {
        let rt = RenderTarget::new(device, Some(size));
        let shader = PostProcessShader::new(&device, PostProcessShaderParams {
            texture: rt.color_tex()
        }).await;
        let mesh = Mesh::quad(&device);

        Self {
            rt,
            shader,
            mesh
        }
    }

    pub fn render<'a, 'b>(&'a mut self, frame: &mut Frame<'b, 'a>) where 'a: 'b {
        self.shader.apply(frame);
        frame.draw_mesh(&self.mesh);
    }
}
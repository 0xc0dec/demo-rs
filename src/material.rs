use wgpu::ShaderModule;
use crate::renderer::Renderer;
use crate::resources::load_string;

pub struct Material {
    shader: wgpu::ShaderModule
}

pub struct MaterialParams {
    pub shader_file_name: &'static str,
}

impl Material {
    pub async fn new(renderer: &Renderer, params: MaterialParams) -> Self {
        let shader_src = load_string(params.shader_file_name).await.unwrap();

        let shader = renderer.device().create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(shader_src.into())
        });

        Self {
            shader
        }
    }

    pub fn shader(&self) -> &ShaderModule { &self.shader }
}
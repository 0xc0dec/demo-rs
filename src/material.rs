use wgpu::{BindGroup, BindGroupLayout, ShaderModule};
use crate::renderer::Renderer;
use crate::resources::load_string;
use crate::texture::Texture;

pub struct Material {
    shader: ShaderModule,
    texture_bind_group_layout: BindGroupLayout,
    texture_bind_group: BindGroup,
}

pub struct MaterialParams {
    pub shader_file_name: &'static str,
    pub texture: Texture,
}

impl Material {
    pub async fn new(renderer: &Renderer, params: MaterialParams) -> Self {
        let shader_src = load_string(params.shader_file_name).await.unwrap();

        let shader = renderer.device().create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(shader_src.into())
        });

        let texture_bind_group_layout =
            renderer.device().create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::D2,
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
                label: None,
            });

        let texture_bind_group = renderer.device().create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &texture_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&params.texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&params.texture.sampler),
                },
            ],
            label: None,
        });

        Self {
            shader,
            texture_bind_group_layout,
            texture_bind_group,
        }
    }

    pub fn shader(&self) -> &ShaderModule { &self.shader }
    pub fn texture_bind_group_layout(&self) -> &BindGroupLayout { &self.texture_bind_group_layout }
}
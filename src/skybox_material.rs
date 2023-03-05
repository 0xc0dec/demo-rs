use wgpu::{BindGroup, RenderPass, RenderPipeline};
use crate::model::{ModelVertex, Vertex};
use crate::driver::Driver;
use crate::resources::load_string;
use crate::texture::Texture;

pub struct SkyboxMaterial {
    pipeline: RenderPipeline,
    texture_bind_group: BindGroup,
}

pub struct SkyboxMaterialParams {
    pub texture: Texture,
}

// TODO Generalize materials ASAP, remove copypasta across different material types
// this is an MVP
impl SkyboxMaterial {
    pub async fn new(driver: &Driver, params: SkyboxMaterialParams) -> Self {
        let shader_src = load_string("skybox.wgsl").await.unwrap();

        let shader = driver.device().create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(shader_src.into()),
        });

        let texture_bind_group_layout =
            driver.device().create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::Cube,
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

        let texture_bind_group = driver.device().create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &texture_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(params.texture.view()),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&params.texture.sampler()),
                },
            ],
            label: None,
        });

        let render_pipeline_layout = driver.device().create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&texture_bind_group_layout],
            push_constant_ranges: &[],
        });

        let pipeline = driver.device().create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[ModelVertex::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: driver.surface_texture_format(),
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None,
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: Texture::DEPTH_FORMAT,
                depth_write_enabled: false,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });

        Self {
            pipeline,
            texture_bind_group
        }
    }
}

pub trait RenderSkyboxMaterial<'a> {
    fn apply_skybox_material(&mut self, material: &'a SkyboxMaterial);
}

impl<'a, 'b> RenderSkyboxMaterial<'b> for RenderPass<'a> where 'b: 'a {
    fn apply_skybox_material(&mut self, material: &'b SkyboxMaterial) {
        self.set_pipeline(&material.pipeline);
        self.set_bind_group(0, &material.texture_bind_group, &[]);
    }
}
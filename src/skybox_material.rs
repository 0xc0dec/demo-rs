use wgpu::{RenderPass, RenderPipeline};
use crate::model::{ModelVertex, Vertex};
use crate::driver::Driver;
use crate::resources::load_string;
use crate::texture::Texture;

pub struct SkyboxMaterial {
    pipeline: RenderPipeline,
}

// TODO Generalize materials ASAP, remove copypasta across different material types
// this is an MVP
impl SkyboxMaterial {
    pub async fn new(driver: &Driver) -> Self {
        let shader_src = load_string("skybox.wgsl").await.unwrap();

        let shader = driver.device().create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(shader_src.into()),
        });

        let render_pipeline_layout = driver.device().create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[],
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
        }
    }
}

pub trait RenderSkyboxMaterial<'a> {
    fn apply_skybox_material(&mut self, material: &'a SkyboxMaterial);
}

impl<'a, 'b> RenderSkyboxMaterial<'b> for RenderPass<'a> where 'b: 'a {
    fn apply_skybox_material(&mut self, material: &'b SkyboxMaterial) {
        self.set_pipeline(&material.pipeline);
    }
}
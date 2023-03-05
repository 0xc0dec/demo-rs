use cgmath::{Matrix4};
use wgpu::{BindGroup, RenderPass, RenderPipeline};
use wgpu::util::DeviceExt;
use crate::camera::Camera;
use crate::model::{ModelVertex, Vertex};
use crate::driver::Driver;
use crate::resources::load_string;
use crate::texture::Texture;
use crate::transform::{Transform};

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct MatricesUniform {
    view_proj: [[f32; 4]; 4],
    world: [[f32; 4]; 4],
}

impl MatricesUniform {
    #[rustfmt::skip]
    const OPENGL_TO_WGPU_MATRIX: Matrix4<f32> = Matrix4::new(
        1.0, 0.0, 0.0, 0.0,
        0.0, 1.0, 0.0, 0.0,
        0.0, 0.0, 0.5, 0.0,
        0.0, 0.0, 0.5, 1.0,
    );

    fn new() -> Self {
        use cgmath::SquareMatrix;
        Self {
            view_proj: Matrix4::identity().into(),
            world: Matrix4::identity().into(),
        }
    }

    fn update(&mut self, camera: &Camera, model_transform: &Transform) {
        self.view_proj = (Self::OPENGL_TO_WGPU_MATRIX * camera.view_proj_matrix()).into();
        self.world = model_transform.matrix().into();
    }
}

pub struct DiffuseMaterial {
    texture_bind_group: BindGroup,
    matrices_uniform: MatricesUniform,
    matrices_uniform_buf: wgpu::Buffer,
    matrices_uniform_bind_group: BindGroup,
    pipeline: RenderPipeline,
}

pub struct DiffuseMaterialParams {
    pub texture: Texture,
}

// TODO ASAP generalize material, remove copypasta across different material types
// this is an MVP
impl DiffuseMaterial {
    pub async fn new(driver: &Driver, params: DiffuseMaterialParams) -> Self {
        let shader_src = load_string("diffuse.wgsl").await.unwrap();

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

        let matrices_uniform = MatricesUniform::new();

        let matrices_uniform_buf = driver.device().create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(&[matrices_uniform]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            }
        );

        let matrices_uniform_bind_group_layout = driver.device().create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }
            ],
            label: None,
        });

        let matrices_uniform_bind_group = driver.device().create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &matrices_uniform_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: matrices_uniform_buf.as_entire_binding(),
                }
            ],
            label: None,
        });

        let render_pipeline_layout = driver.device().create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[
                &texture_bind_group_layout,
                &matrices_uniform_bind_group_layout
            ],
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
                depth_write_enabled: true,
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
            texture_bind_group,
            matrices_uniform,
            matrices_uniform_buf,
            matrices_uniform_bind_group,
            pipeline,
        }
    }

    pub fn update(&mut self, driver: &Driver, camera: &Camera, transform: &Transform) {
        self.matrices_uniform.update(camera, transform);
        driver.queue().write_buffer(
            &self.matrices_uniform_buf,
            0,
            bytemuck::cast_slice(&[self.matrices_uniform]),
        );
    }
}

pub trait RenderDiffuseMaterial<'a> {
    fn apply_material(&mut self, material: &'a DiffuseMaterial);
}

impl<'a, 'b> RenderDiffuseMaterial<'b> for RenderPass<'a> where 'b: 'a {
    fn apply_material(&mut self, material: &'b DiffuseMaterial) {
        self.set_pipeline(&material.pipeline);
        self.set_bind_group(0, &material.texture_bind_group, &[]);
        self.set_bind_group(1, &material.matrices_uniform_bind_group, &[]);
    }
}
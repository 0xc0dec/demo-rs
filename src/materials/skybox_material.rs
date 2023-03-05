use cgmath::{Matrix4, SquareMatrix, Vector3};
use wgpu::{BindGroup, RenderPass, RenderPipeline};
use wgpu::util::DeviceExt;
use crate::camera::Camera;
use crate::model::{ModelVertex, Vertex};
use crate::driver::Driver;
use crate::resources::load_string;
use crate::texture::Texture;

pub struct SkyboxMaterial {
    pipeline: RenderPipeline,
    texture_bind_group: BindGroup,
    data_uniform: DataUniform,
    data_uniform_buf: wgpu::Buffer,
    data_uniform_bind_group: BindGroup,
}

pub struct SkyboxMaterialParams {
    pub texture: Texture,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct DataUniform {
    proj_mat_inv: [[f32; 4]; 4],
    // Couldn't make it work with Matrix3, probably something to do with alignment and padding
    view_mat: [[f32; 4]; 4],
}

impl DataUniform {
    #[rustfmt::skip]
    const OPENGL_TO_WGPU_MATRIX: Matrix4<f32> = Matrix4::new(
        1.0, 0.0, 0.0, 0.0,
        0.0, 1.0, 0.0, 0.0,
        0.0, 0.0, 0.5, 0.0,
        0.0, 0.0, 0.5, 1.0,
    );

    fn new() -> Self {
        Self {
            proj_mat_inv: Matrix4::identity().into(),
            view_mat: Matrix4::identity().into(),
        }
    }

    fn update(&mut self, camera: &Camera) {
        self.view_mat = camera.view_matrix().into();
        self.proj_mat_inv = (Self::OPENGL_TO_WGPU_MATRIX * camera.proj_matrix()).invert().unwrap().into();
    }
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

        let data_uniform = DataUniform::new();

        let data_uniform_buf = driver.device().create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(&[data_uniform]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            }
        );

        let data_uniform_bind_group_layout = driver.device().create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
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

        let data_uniform_bind_group = driver.device().create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &data_uniform_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: data_uniform_buf.as_entire_binding(),
                }
            ],
            label: None,
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
            bind_group_layouts: &[
                &data_uniform_bind_group_layout,
                &texture_bind_group_layout
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
            texture_bind_group,
            data_uniform,
            data_uniform_buf,
            data_uniform_bind_group
        }
    }

    pub fn update(&mut self, driver: &Driver, camera: &Camera) {
        self.data_uniform.update(camera);
        driver.queue().write_buffer(
            &self.data_uniform_buf,
            0,
            bytemuck::cast_slice(&[self.data_uniform]),
        );
    }
}

pub trait RenderSkyboxMaterial<'a> {
    fn apply_skybox_material(&mut self, material: &'a SkyboxMaterial);
}

impl<'a, 'b> RenderSkyboxMaterial<'b> for RenderPass<'a> where 'b: 'a {
    fn apply_skybox_material(&mut self, material: &'b SkyboxMaterial) {
        self.set_pipeline(&material.pipeline);
        self.set_bind_group(0, &material.data_uniform_bind_group, &[]);
        self.set_bind_group(1, &material.texture_bind_group, &[]);
    }
}
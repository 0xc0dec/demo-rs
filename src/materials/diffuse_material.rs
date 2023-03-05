use cgmath::{Matrix4};
use wgpu::{BindGroup, RenderPipeline};
use wgpu::util::DeviceExt;
use crate::camera::Camera;
use crate::model::{ModelVertex, Vertex};
use crate::driver::Driver;
use crate::materials::{new_render_pipeline, new_texture_bind_group, RenderPipelineParams};
use super::Material;
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
        let (texture_bind_group_layout, texture_bind_group) =
            new_texture_bind_group(driver, &params.texture, wgpu::TextureViewDimension::D2);

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

        let pipeline = new_render_pipeline(
            driver,
            RenderPipelineParams {
                shader_file_name: "diffuse.wgsl",
                depth_write: true,
                bind_group_layouts: &[
                    &texture_bind_group_layout,
                    &matrices_uniform_bind_group_layout
                ],
                vertex_buffer_layouts: &[ModelVertex::desc()]
            }
        ).await;

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

impl Material for DiffuseMaterial {
    fn apply<'a, 'b>(&'a mut self, pass: &mut wgpu::RenderPass<'b>) where 'a: 'b {
        pass.set_pipeline(&self.pipeline);
        pass.set_bind_group(0, &self.texture_bind_group, &[]);
        pass.set_bind_group(1, &self.matrices_uniform_bind_group, &[]);
    }
}

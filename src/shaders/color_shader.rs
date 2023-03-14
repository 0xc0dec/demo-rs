use cgmath::{Matrix4};
use wgpu::{BindGroup, RenderPipeline};
use crate::camera::Camera;
use crate::model::{ModelVertex, Vertex};
use crate::device::{Device, Frame};
use crate::shaders::utils::*;
use super::Shader;
use crate::transform::{Transform};

pub struct ColorShader {
    matrices_uniform: MatricesUniform,
    matrices_uniform_buf: wgpu::Buffer,
    matrices_uniform_bind_group: BindGroup,
    pipeline: RenderPipeline,
}

impl ColorShader {
    pub async fn new(device: &Device) -> Self {
        let matrices_uniform = MatricesUniform::new();
        let (
            matrices_uniform_bind_group_layout,
            matrices_uniform_bind_group,
            matrices_uniform_buf
        ) = new_uniform_bind_group(device, bytemuck::cast_slice(&[matrices_uniform]));

        let pipeline = new_render_pipeline(
            device,
            RenderPipelineParams {
                shader_file_name: "color.wgsl",
                depth_write: true,
                depth_enabled: true,
                bind_group_layouts: &[
                    &matrices_uniform_bind_group_layout
                ],
                vertex_buffer_layouts: &[ModelVertex::desc()]
            }
        ).await;

        Self {
            matrices_uniform,
            matrices_uniform_buf,
            matrices_uniform_bind_group,
            pipeline,
        }
    }

    pub fn update(&mut self, device: &Device, camera: &Camera, transform: &Transform) {
        self.matrices_uniform.update(camera, transform);
        device.queue().write_buffer(
            &self.matrices_uniform_buf,
            0,
            bytemuck::cast_slice(&[self.matrices_uniform]),
        );
    }
}

impl<'a, 'b> Shader<'a, 'b> for ColorShader where 'a: 'b  {
    fn apply(&'a mut self, frame: &mut Frame<'b, 'a>) {
        frame.set_pipeline(&self.pipeline);
        frame.set_bind_group(0, &self.matrices_uniform_bind_group, &[]);
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct MatricesUniform {
    view_proj: [[f32; 4]; 4],
    world: [[f32; 4]; 4],
}

impl MatricesUniform {
    // TODO Remove copypasta
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
use wgpu::{BindGroup, RenderPipeline};

use crate::components::Camera;
use crate::components::Transform;
use crate::device::Device;
use crate::math::{Mat4, OPENGL_TO_WGPU_MATRIX};
use crate::mesh::MeshVertex;
use crate::shaders::utils::*;

pub struct ColorShader {
    matrices_uniform: MatricesUniform,
    matrices_uniform_buf: wgpu::Buffer,
    matrices_uniform_bind_group: BindGroup,
    pipeline: RenderPipeline,
}

impl ColorShader {
    pub async fn new(device: &Device) -> Self {
        let matrices_uniform = MatricesUniform::new();
        let (matrices_uniform_bind_group_layout, matrices_uniform_bind_group, matrices_uniform_buf) =
            new_uniform_bind_group(device, bytemuck::cast_slice(&[matrices_uniform]));

        let shader_module = new_shader_module(device, "color.wgsl").await;

        let pipeline = new_render_pipeline(
            device,
            RenderPipelineParams {
                shader_module,
                depth_write: true,
                depth_enabled: true,
                bind_group_layouts: &[&matrices_uniform_bind_group_layout],
                vertex_buffer_layouts: &[MeshVertex::buffer_layout()],
            },
        )
        .await;

        Self {
            matrices_uniform,
            matrices_uniform_buf,
            matrices_uniform_bind_group,
            pipeline,
        }
    }

    pub fn update_uniforms(
        &mut self,
        device: &Device,
        camera: (&Camera, &Transform),
        transform: &Transform,
    ) {
        self.matrices_uniform.update(camera, transform);
        device.queue().write_buffer(
            &self.matrices_uniform_buf,
            0,
            bytemuck::cast_slice(&[self.matrices_uniform]),
        );
    }

    pub fn apply<'a>(&'a mut self, encoder: &mut wgpu::RenderBundleEncoder<'a>) {
        encoder.set_pipeline(&self.pipeline);
        encoder.set_bind_group(0, &self.matrices_uniform_bind_group, &[]);
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct MatricesUniform {
    view_proj: [[f32; 4]; 4],
    world: [[f32; 4]; 4],
}

impl MatricesUniform {
    fn new() -> Self {
        Self {
            view_proj: Mat4::identity().into(),
            world: Mat4::identity().into(),
        }
    }

    fn update(&mut self, camera: (&Camera, &Transform), mesh_transform: &Transform) {
        self.view_proj =
            (OPENGL_TO_WGPU_MATRIX * camera.0.proj_matrix() * camera.1.view_matrix()).into();
        self.world = mesh_transform.matrix().into();
    }
}

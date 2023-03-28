use super::Shader;
use crate::components::Camera;
use crate::components::Transform;
use crate::device::Device;
use crate::math::Mat4;
use crate::mesh::{MeshVertex, Vertex};
use crate::shaders::utils::*;
use wgpu::{BindGroup, RenderPipeline};

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

        let pipeline = new_render_pipeline(
            device,
            RenderPipelineParams {
                shader_file_name: "color.wgsl",
                depth_write: true,
                depth_enabled: true,
                bind_group_layouts: &[&matrices_uniform_bind_group_layout],
                vertex_buffer_layouts: &[MeshVertex::desc()],
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

    pub fn update(
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
}

impl Shader for ColorShader {
    fn apply<'a>(&'a mut self, encoder: &mut wgpu::RenderBundleEncoder<'a>) {
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
    // TODO Remove copypasta
    #[rustfmt::skip]
    const OPENGL_TO_WGPU_MATRIX: Mat4 = Mat4::new(
        1.0, 0.0, 0.0, 0.0,
        0.0, 1.0, 0.0, 0.0,
        0.0, 0.0, 0.5, 0.5,
        0.0, 0.0, 0.0, 1.0,
    );

    fn new() -> Self {
        Self {
            view_proj: Mat4::identity().into(),
            world: Mat4::identity().into(),
        }
    }

    fn update(&mut self, camera: (&Camera, &Transform), mesh_transform: &Transform) {
        self.view_proj =
            (Self::OPENGL_TO_WGPU_MATRIX * camera.0.proj_matrix() * camera.1.view_matrix()).into();
        self.world = mesh_transform.matrix().into();
    }
}

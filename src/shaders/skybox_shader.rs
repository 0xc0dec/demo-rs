use crate::components::{Camera, Transform};
use crate::device::Device;
use crate::math::{Mat4, OPENGL_TO_WGPU_MATRIX};
use crate::mesh::{MeshVertex, Vertex};
use crate::shaders::utils::*;
use crate::shaders::Shader;
use crate::texture::Texture;

pub struct SkyboxShader {
    pipeline: wgpu::RenderPipeline,
    texture_bind_group: wgpu::BindGroup,
    data_uniform: DataUniform,
    data_uniform_buf: wgpu::Buffer,
    data_uniform_bind_group: wgpu::BindGroup,
}

pub struct SkyboxShaderParams {
    pub texture: Texture,
}

impl SkyboxShader {
    pub async fn new(device: &Device, params: SkyboxShaderParams) -> Self {
        let data_uniform = DataUniform::new();

        let (data_uniform_bind_group_layout, data_uniform_bind_group, data_uniform_buf) =
            new_uniform_bind_group(device, bytemuck::cast_slice(&[data_uniform]));

        let (texture_bind_group_layout, texture_bind_group) =
            new_texture_bind_group(device, &params.texture, wgpu::TextureViewDimension::Cube);

        let pipeline = new_render_pipeline(
            device,
            RenderPipelineParams {
                shader_file_name: "skybox.wgsl",
                depth_write: false,
                depth_enabled: true,
                bind_group_layouts: &[&data_uniform_bind_group_layout, &texture_bind_group_layout],
                vertex_buffer_layouts: &[MeshVertex::desc()],
            },
        )
        .await;

        Self {
            pipeline,
            texture_bind_group,
            data_uniform,
            data_uniform_buf,
            data_uniform_bind_group,
        }
    }

    pub fn update(&mut self, device: &Device, camera: (&Camera, &Transform)) {
        self.data_uniform.update(camera);
        device.queue().write_buffer(
            &self.data_uniform_buf,
            0,
            bytemuck::cast_slice(&[self.data_uniform]),
        );
    }
}

impl Shader for SkyboxShader {
    fn apply<'a>(&'a mut self, encoder: &mut wgpu::RenderBundleEncoder<'a>) {
        encoder.set_pipeline(&self.pipeline);
        encoder.set_bind_group(0, &self.data_uniform_bind_group, &[]);
        encoder.set_bind_group(1, &self.texture_bind_group, &[]);
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct DataUniform {
    proj_mat_inv: [[f32; 4]; 4],
    // Couldn't make it work with Matrix3, probably something to do with alignment and padding
    view_mat: [[f32; 4]; 4],
}

impl DataUniform {
    fn new() -> Self {
        Self {
            proj_mat_inv: Mat4::identity().into(),
            view_mat: Mat4::identity().into(),
        }
    }

    fn update(&mut self, camera: (&Camera, &Transform)) {
        self.view_mat = camera.1.view_matrix().into();
        self.proj_mat_inv = (OPENGL_TO_WGPU_MATRIX * camera.0.proj_matrix())
            .try_inverse()
            .unwrap()
            .into();
    }
}

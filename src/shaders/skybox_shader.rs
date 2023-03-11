use cgmath::{Matrix4, SquareMatrix};
use wgpu::{BindGroup, RenderPipeline};
use crate::camera::Camera;
use crate::model::{ModelVertex, Vertex};
use crate::device::{Device, Frame};
use crate::shaders::Shader;
use crate::shaders::utils::*;
use crate::texture::Texture;

pub struct SkyboxShader {
    pipeline: RenderPipeline,
    texture_bind_group: BindGroup,
    data_uniform: DataUniform,
    data_uniform_buf: wgpu::Buffer,
    data_uniform_bind_group: BindGroup,
}

pub struct SkyboxShaderParams {
    pub texture: Texture,
}

impl SkyboxShader {
    pub async fn new(gfx: &Device, params: SkyboxShaderParams) -> Self {
        let data_uniform = DataUniform::new();

        let (
            data_uniform_bind_group_layout,
            data_uniform_bind_group,
            data_uniform_buf
        ) = new_uniform_bind_group(gfx, bytemuck::cast_slice(&[data_uniform]));

        let (
            texture_bind_group_layout,
            texture_bind_group
        ) = new_texture_bind_group(gfx, &params.texture, wgpu::TextureViewDimension::Cube);

        let pipeline = new_render_pipeline(
            gfx,
            RenderPipelineParams {
                shader_file_name: "skybox.wgsl",
                depth_write: false,
                depth_enabled: true,
                bind_group_layouts: &[
                    &data_uniform_bind_group_layout,
                    &texture_bind_group_layout
                ],
                vertex_buffer_layouts: &[ModelVertex::desc()]
            }
        ).await;

        Self {
            pipeline,
            texture_bind_group,
            data_uniform,
            data_uniform_buf,
            data_uniform_bind_group
        }
    }

    pub fn update(&mut self, gfx: &Device, camera: &Camera) {
        self.data_uniform.update(camera);
        gfx.queue().write_buffer(
            &self.data_uniform_buf,
            0,
            bytemuck::cast_slice(&[self.data_uniform]),
        );
    }
}

impl<'a, 'b> Shader<'a, 'b> for SkyboxShader where 'a: 'b {
    fn apply(&'a mut self, frame: &mut Frame<'b>) {
        frame.bundle_encoder.set_pipeline(&self.pipeline);
        frame.bundle_encoder.set_bind_group(0, &self.data_uniform_bind_group, &[]);
        frame.bundle_encoder.set_bind_group(1, &self.texture_bind_group, &[]);
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

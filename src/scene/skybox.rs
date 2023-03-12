use crate::camera::Camera;
use crate::device::{Device, Frame};
use crate::model::{DrawModel, Mesh};
use crate::shaders::{Shader, SkyboxShader, SkyboxShaderParams};
use crate::texture::Texture;

pub struct Skybox {
    mesh: Mesh,
    shader: SkyboxShader,
}

impl Skybox {
    pub async fn new(device: &Device) -> Self {
        let texture = Texture::new_cube_from_file("skybox_bgra.dds", device).await.unwrap();

        Self {
            mesh: Mesh::quad(device),
            shader: SkyboxShader::new(device, SkyboxShaderParams { texture }).await,
        }
    }

    pub fn render<'a, 'b, 'c>(&'c mut self, device: &Device, camera: &Camera, frame: &mut Frame<'b, 'a>)
        where 'a: 'b,
              'c: 'a
    {
        self.shader.update(&device, camera);
        self.shader.apply(frame);
        frame.draw_mesh(&self.mesh);
    }
}
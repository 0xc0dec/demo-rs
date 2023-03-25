use bevy_ecs::prelude::{Commands, Component};
use bevy_ecs::system::{NonSend};
use crate::components::camera::Camera;
use crate::device::{Device};
use crate::model::{DrawModel, Mesh};
use crate::shaders::{Shader, SkyboxShader, SkyboxShaderParams};
use crate::texture::Texture;

#[derive(Component)]
pub struct Skybox {
    mesh: Mesh,
    shader: SkyboxShader,
}

impl Skybox {
    pub fn spawn(mut commands: Commands, device: NonSend<Device>) {
        let skybox = pollster::block_on(Skybox::new(&device));
        commands.spawn((skybox,));
    }

    // TODO Use RenderModel
    async fn new(device: &Device) -> Self {
        let texture = Texture::new_cube_from_file("skybox_bgra.dds", device)
            .await
            .unwrap();
        let shader = SkyboxShader::new(device, SkyboxShaderParams { texture })
            .await;
        let mesh = Mesh::quad(device);

        Self {
            mesh,
            shader,
        }
    }

    pub fn render<'a>(
        &'a mut self,
        device: &Device,
        camera: &Camera,
        encoder: &mut wgpu::RenderBundleEncoder<'a>,
    ) {
        self.shader.update(&device, camera);
        self.shader.apply(encoder);
        encoder.draw_mesh(&self.mesh);
    }
}

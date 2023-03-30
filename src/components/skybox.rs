use crate::components::transform::Transform;
use crate::components::{MeshRenderer, ShaderVariant, RenderOrder};
use crate::device::Device;
use crate::mesh::Mesh;
use crate::render_tags::RenderTags;
use crate::shaders::{SkyboxShader, SkyboxShaderParams};
use crate::texture::Texture;
use bevy_ecs::prelude::*;
use bevy_ecs::system::NonSend;

#[derive(Component)]
pub struct Skybox;

impl Skybox {
    pub fn spawn(mut commands: Commands, device: NonSend<Device>) {
        pollster::block_on(async {
            let texture = Texture::new_cube_from_file("skybox_bgra.dds", &device)
                .await
                .unwrap();
            let shader = SkyboxShader::new(&device, SkyboxShaderParams { texture: &texture }).await;
            let mesh = Mesh::quad(&device);
            let renderer = MeshRenderer::new(
                mesh,
                ShaderVariant::Skybox(shader),
                RenderTags::SCENE,
            );

            let transform = Transform::default();

            commands.spawn((Skybox, RenderOrder(-100), renderer, transform));
        });
    }
}

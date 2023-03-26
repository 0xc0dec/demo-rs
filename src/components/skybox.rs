use bevy_ecs::prelude::{Commands, Component};
use bevy_ecs::system::{NonSend};
use crate::components::{ModelShader, RenderOrder, ModelRenderer};
use crate::device::{Device};
use crate::model::{Model};
use crate::shaders::{SkyboxShader, SkyboxShaderParams};
use crate::texture::Texture;
use crate::components::transform::Transform;

#[derive(Component)]
pub struct Skybox;

impl Skybox {
    pub fn spawn(mut commands: Commands, device: NonSend<Device>) {
        pollster::block_on(async {
            let texture = Texture::new_cube_from_file("skybox_bgra.dds", &device)
                .await
                .unwrap();
            let shader = SkyboxShader::new(&device, SkyboxShaderParams { texture })
                .await;
            let model = Model::quad(&device);
            let render_model = ModelRenderer {
                shader: ModelShader::Skybox(shader),
                model,
            };

            let transform = Transform::default();

            commands.spawn((
                Skybox,
                RenderOrder(-100),
                render_model,
                transform
            ));
        });
    }
}

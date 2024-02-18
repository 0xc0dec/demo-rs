use bevy_ecs::prelude::*;

use crate::assets::Mesh;
use crate::components::{Material, MeshRenderer, RenderOrder, RenderTags, Transform};
use crate::render_tags::RENDER_TAG_SCENE;
use crate::resources::{Assets, Device};

#[derive(Component)]
pub struct Skybox;

impl Skybox {
    pub fn spawn(mut commands: Commands, device: Res<Device>, assets: Res<Assets>) {
        let material = Material::skybox(&device, &assets, &assets.skybox_tex);
        let mesh = Mesh::quad(&device);
        let renderer = MeshRenderer::new(mesh);
        let transform = Transform::default();

        commands.spawn((
            Skybox,
            renderer,
            material,
            transform,
            RenderOrder(-100),
            RenderTags(RENDER_TAG_SCENE),
        ));
    }
}

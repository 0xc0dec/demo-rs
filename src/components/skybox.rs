use crate::assets;
use bevy_ecs::prelude::*;

use crate::components::{Material, Mesh, RenderOrder, RenderTags, Transform};
use crate::render_tags::RENDER_TAG_SCENE;
use crate::resources::{Assets, Device};

#[derive(Component)]
pub struct Skybox;

impl Skybox {
    pub fn spawn(mut commands: Commands, device: Res<Device>, assets: Res<Assets>) {
        let material = Material::skybox(&device, &assets, &assets.skybox_tex);
        let mesh = Mesh(assets::Mesh::quad(&device));
        let transform = Transform::default();

        commands.spawn((
            Skybox,
            mesh,
            material,
            transform,
            RenderOrder(-100),
            RenderTags(RENDER_TAG_SCENE),
        ));
    }
}

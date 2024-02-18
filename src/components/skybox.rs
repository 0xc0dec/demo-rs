use bevy_ecs::prelude::*;

use crate::assets::Mesh;
use crate::components::render_tags::RenderTags;
use crate::components::transform::Transform;
use crate::components::{Material, MeshRenderer, RenderOrder};
use crate::materials::SkyboxMaterial;
use crate::render_tags::RENDER_TAG_SCENE;
use crate::resources::{Assets, Device};

#[derive(Component)]
pub struct Skybox;

impl Skybox {
    pub fn spawn(mut commands: Commands, device: Res<Device>, assets: Res<Assets>) {
        let shader = SkyboxMaterial::new(&device, &assets, &assets.skybox_tex);
        let mesh = Mesh::quad(&device);
        let renderer = MeshRenderer::new(mesh, Material::Skybox(shader));
        let transform = Transform::default();

        commands.spawn((
            Skybox,
            renderer,
            transform,
            RenderOrder(-100),
            RenderTags(RENDER_TAG_SCENE),
        ));
    }
}

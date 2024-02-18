use bevy_ecs::prelude::*;

use crate::assets::Assets;
use crate::components::render_tags::RenderTags;
use crate::components::transform::Transform;
use crate::components::{Material, MeshRenderer, RenderOrder};
use crate::device::Device;
use crate::materials::SkyboxMaterial;
use crate::mesh::Mesh;
use crate::render_tags::RENDER_TAG_SCENE;

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

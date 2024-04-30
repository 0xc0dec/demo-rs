use hecs::{Entity, World};

use crate::new::{Assets, Device, Material, Mesh, RENDER_TAG_SCENE, RenderTags, Transform};

pub struct Skybox;

impl Skybox {
    pub fn spawn(device: &Device, assets: &Assets, world: &mut World) -> Entity {
        let material = Material::skybox(&device, &assets, &assets.skybox_tex);
        let mesh = Mesh::quad(&device);
        let transform = Transform::default();

        world.spawn((transform, mesh, material, RenderTags(RENDER_TAG_SCENE)))
    }
}

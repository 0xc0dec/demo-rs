use crate::new::{Assets, Device, Material, Mesh, RENDER_TAG_SCENE, RenderTags, Transform};
use crate::new::render_order::RenderOrder;

pub struct Skybox;

impl Skybox {
    pub fn spawn(
        device: &Device,
        assets: &Assets,
    ) -> (Mesh, Material, Transform, RenderOrder, RenderTags) {
        let material = Material::skybox(&device, &assets, &assets.skybox_tex);
        let mesh = Mesh::quad(&device);
        let transform = Transform::default();

        (
            mesh,
            material,
            transform,
            RenderOrder(-100),
            RenderTags(RENDER_TAG_SCENE),
        )
    }
}

use hecs::{Entity, World};

use crate::new::{
    assets, Assets, Device, Material, Player, RENDER_TAG_HIDDEN, RENDER_TAG_SCENE, RenderTags, Transform,
    Vec3,
};

pub struct PlayerTarget;

// TODO Rename to smth like "raycast target"
impl PlayerTarget {
    pub fn spawn(device: &Device, assets: &Assets, world: &mut World) -> Entity {
        // TODO Load in assets
        let mesh = pollster::block_on(async { assets::Mesh::from_file("cube.obj", device).await });
        let material = Material::color(device, assets);
        let transform = Transform::default();

        world.spawn((
            PlayerTarget,
            transform,
            mesh,
            material,
            RenderTags(RENDER_TAG_HIDDEN),
        ))
    }

    pub fn update(world: &mut World) {
        let mut q = world.query::<(&PlayerTarget, &mut Transform, &mut RenderTags)>();
        let (_, (_, tr, tags)) = q.iter().next().unwrap();

        let mut q = world.query::<(&Player, &Transform)>();
        let (_, (player, player_tr)) = q.iter().next().unwrap();

        if let Some(player_target_pt) = player.target_pt() {
            let dist_to_camera = (player_tr.position() - player_target_pt).magnitude();
            let scale = (dist_to_camera / 10.0).min(0.1).max(0.01);
            tags.0 = RENDER_TAG_SCENE;
            tr.set_position(player_target_pt);
            tr.set_scale(Vec3::from_element(scale));
        } else {
            tags.0 = RENDER_TAG_HIDDEN;
        }
    }
}

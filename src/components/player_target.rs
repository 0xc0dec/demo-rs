use bevy_ecs::prelude::*;

use crate::assets;
use crate::components::{
    Material, Mesh, Player, RenderTags, Transform, RENDER_TAG_HIDDEN, RENDER_TAG_SCENE,
};
use crate::math::Vec3;
use crate::resources::{Assets, Device};

#[derive(Component)]
pub struct PlayerTarget;

impl PlayerTarget {
    pub fn spawn(device: Res<Device>, assets: Res<Assets>, mut commands: Commands) {
        // TODO Load in assets
        let mesh = Mesh(pollster::block_on(assets::Mesh::from_file(
            "cube.obj", &device,
        )));
        let material = Material::color(&device, &assets);
        let transform = Transform::default();

        commands.spawn((
            PlayerTarget,
            transform,
            mesh,
            material,
            RenderTags(RENDER_TAG_HIDDEN),
        ));
    }

    pub fn update(
        player: Query<(&Player, &Transform), Without<PlayerTarget>>,
        mut target: Query<(Entity, &mut Transform), With<PlayerTarget>>,
        mut commands: Commands,
    ) {
        let (player, player_tr) = player.single();
        let (target_ent, mut target_tr) = target.single_mut();

        if let Some(player_focus_pt) = player.focus_point() {
            let dist_to_camera = (player_tr.position() - player_focus_pt).magnitude();
            let scale = (dist_to_camera / 10.0).min(0.1).max(0.01);

            commands
                .entity(target_ent)
                .insert(RenderTags(RENDER_TAG_SCENE));
            target_tr.set_position(player_focus_pt);
            target_tr.set_scale(Vec3::from_element(scale));
        } else {
            commands
                .entity(target_ent)
                .insert(RenderTags(RENDER_TAG_HIDDEN));
        }
    }
}

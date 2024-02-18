use bevy_ecs::prelude::*;

use crate::assets::Assets;
use crate::components::render_tags::RenderTags;
use crate::components::{Material, MeshRenderer, Player, Transform};
use crate::device::Device;
use crate::materials::ColorMaterial;
use crate::math::Vec3;
use crate::mesh::Mesh;
use crate::render_tags::{RENDER_TAG_HIDDEN, RENDER_TAG_SCENE};

#[derive(Component)]
pub struct PlayerTarget;

// TODO Rename to smth like "raycast target"
impl PlayerTarget {
    pub fn spawn(device: Res<Device>, assets: Res<Assets>, mut commands: Commands) {
        let transform = Transform::default();
        let (shader, mesh) = pollster::block_on(async {
            // TODO Load in assets
            let mesh = Mesh::from_file("cube.obj", &device).await;
            let shader = ColorMaterial::new(&device, &assets);
            (shader, mesh)
        });
        let renderer = MeshRenderer::new(mesh, Material::Color(shader));

        commands.spawn((
            PlayerTarget,
            transform,
            renderer,
            RenderTags(RENDER_TAG_HIDDEN),
        ));
    }

    pub fn update(
        // Without this Without it crashes :|
        player: Query<(&Player, &Transform), Without<PlayerTarget>>,
        mut target: Query<(Entity, &mut Transform), With<PlayerTarget>>,
        mut commands: Commands,
    ) {
        let (player, player_transform) = player.single();
        let (target_entity, mut target_transform) = target.single_mut();

        if let Some(player_target_pt) = player.target_pt() {
            let dist_to_camera = (player_transform.position() - player_target_pt).magnitude();
            let scale = (dist_to_camera / 10.0).min(0.1).max(0.01);

            commands
                .entity(target_entity)
                .insert(RenderTags(RENDER_TAG_SCENE));
            target_transform.set_position(player_target_pt);
            target_transform.set_scale(Vec3::from_element(scale));
        } else {
            commands
                .entity(target_entity)
                .insert(RenderTags(RENDER_TAG_HIDDEN));
        }
    }
}

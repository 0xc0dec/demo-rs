use bevy_ecs::prelude::*;

use crate::assets;
use crate::components::{Camera, Material, Mesh, Player, RenderOrder, RenderTags, Transform};
use crate::render_tags::{RENDER_TAG_DEBUG_UI, RENDER_TAG_POST_PROCESS};
use crate::resources::{Assets, Device};

#[derive(Component)]
pub struct PostProcessor {
    size: assets::TextureSize,
}

impl PostProcessor {
    pub fn spawn(
        mut commands: Commands,
        player: Query<&Camera, With<Player>>,
        device: Res<Device>,
        assets: Res<Assets>,
    ) {
        // We know we need the player camera
        let source_camera_rt = player.single().target().as_ref().unwrap();
        let material = Material::post_process(&device, &assets, source_camera_rt.color_tex());
        let mesh = Mesh(assets::Mesh::quad(&device));
        let transform = Transform::default();
        let pp = PostProcessor {
            size: source_camera_rt.color_tex().size(),
        };

        commands.spawn((
            pp,
            mesh,
            material,
            transform,
            RenderTags(RENDER_TAG_POST_PROCESS),
        ));

        // Camera for rendering the quad (and debug UI for that matter)
        let camera = Camera::new(1.0, RENDER_TAG_POST_PROCESS | RENDER_TAG_DEBUG_UI, None);
        let transform = Transform::default();
        commands.spawn((RenderOrder(100), camera, transform));
    }

    pub fn update(
        device: Res<Device>,
        mut pp: Query<(Entity, &mut PostProcessor, &mut Mesh, &mut Material)>,
        player_cam: Query<&Camera, With<Player>>,
        assets: Res<Assets>,
        mut commands: Commands,
    ) {
        if let Some(pp) = pp.iter_mut().next().as_mut() {
            let source_camera_rt = player_cam.single().target().as_ref().unwrap();

            if source_camera_rt.color_tex().size() != pp.1.size {
                pp.1.size = source_camera_rt.color_tex().size();
                // TODO We should NOT be re-creating the material.
                commands.entity(pp.0).insert(Material::post_process(
                    &device,
                    &assets,
                    source_camera_rt.color_tex(),
                ));
            }
        }
    }
}

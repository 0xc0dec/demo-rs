use crate::components::{MeshRenderer, ShaderVariant, Player, Transform};
use crate::device::Device;
use crate::math::Vec3;
use crate::mesh::Mesh;
use crate::render_tags::RenderTags;
use crate::shaders::ColorShader;
use bevy_ecs::prelude::*;

#[derive(Component)]
pub struct PlayerTarget;

impl PlayerTarget {
    pub fn spawn(mut commands: Commands, device: Res<Device>) {
        let transform = Transform::default();

        let (shader, mesh) = pollster::block_on(async {
            let mesh = Mesh::from_file("cube.obj", &device).await;
            let shader = ColorShader::new(&device).await;
            (shader, mesh)
        });

        let renderer = MeshRenderer::new(
            mesh,
            ShaderVariant::Color(shader),
            RenderTags::HIDDEN,
        );

        commands.spawn((PlayerTarget, transform, renderer));
    }

    pub fn update(
        // Without this Without it crashes :|
        player: Query<(&Player, &Transform), Without<PlayerTarget>>,
        mut target: Query<(&mut Transform, &mut MeshRenderer), With<PlayerTarget>>,
    ) {
        let (player, player_transform) = player.single();
        let (mut target_transform, mut target_renderer) = target.single_mut();

        if let Some(player_raycast_pt) = player.target_pt() {
            let dist_to_camera = (player_transform.position() - player_raycast_pt).magnitude();
            let scale = (dist_to_camera / 10.0).min(0.1).max(0.01);

            target_renderer.set_tags(RenderTags::SCENE);
            target_transform.set_position(player_raycast_pt);
            target_transform.set_scale(Vec3::from_element(scale));
        } else {
            target_renderer.set_tags(RenderTags::HIDDEN);
        }
    }
}

use bevy_ecs::prelude::*;

use crate::assets::*;
use crate::components::*;
use crate::materials::DiffuseMaterial;
use crate::math::Vec3;
use crate::render_tags::RENDER_TAG_SCENE;
use crate::resources::{Assets, Device, Input, PhysicsWorld};

#[derive(Component)]
pub struct FreeBox;

impl FreeBox {
    pub fn spawn(
        mut commands: Commands,
        device: Res<Device>,
        mut physics: ResMut<PhysicsWorld>,
        assets: Res<Assets>,
    ) {
        let pos = Vec3::y_axis().xyz() * 5.0;
        commands.spawn(Self::new_components(pos, &device, &mut physics, &assets));
    }

    pub fn spawn_by_player(
        player: Query<&Transform, With<Player>>,
        mut commands: Commands,
        device: Res<Device>,
        mut physics: ResMut<PhysicsWorld>,
        input: Res<Input>,
        assets: Res<Assets>,
    ) {
        if input.space_just_pressed {
            let player_transform = player.single();
            let pos = player_transform.position() + player_transform.forward().xyz() * 5.0;
            commands.spawn(Self::new_components(pos, &device, &mut physics, &assets));
        }
    }

    fn new_components(
        pos: Vec3,
        device: &Device,
        physics: &mut PhysicsWorld,
        assets: &Assets,
    ) -> (FreeBox, PhysicsBody, MeshRenderer, Transform, RenderTags) {
        let (shader, mesh) = pollster::block_on(async {
            let shader = DiffuseMaterial::new(device, &assets, &assets.stone_tex);
            // TODO Load in assets
            let mesh = Mesh::from_file("cube.obj", device).await;
            (shader, mesh)
        });
        let scale = Vec3::from_element(1.0);
        let body = PhysicsBody::new(
            PhysicsBodyParams {
                pos,
                scale,
                rotation_axis: Vec3::identity(),
                rotation_angle: 0.0,
                movable: true,
            },
            physics,
        );
        let renderer = MeshRenderer::new(mesh, Material::Diffuse(shader));
        let transform = Transform::new(pos, scale);

        (
            FreeBox,
            body,
            renderer,
            transform,
            RenderTags(RENDER_TAG_SCENE),
        )
    }
}

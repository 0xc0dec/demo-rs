use bevy_ecs::prelude::*;

use crate::assets;
use crate::components::{
    Material, Mesh, PhysicsBody, PhysicsBodyParams, Player, RENDER_TAG_SCENE, RenderTags, Transform,
};
use crate::math::Vec3;
use crate::resources::{Assets, Device, Input, PhysicsWorld};

#[derive(Component)]
pub struct FreeBox;

impl FreeBox {
    pub fn spawn_sample(
        device: Res<Device>,
        assets: Res<Assets>,
        mut physics: ResMut<PhysicsWorld>,
        mut commands: Commands,
    ) {
        let pos = Vec3::y_axis().xyz() * 5.0;
        commands.spawn(Self::components(pos, &device, &mut physics, &assets));
    }

    pub fn spawn_by_player(
        device: Res<Device>,
        player: Query<&Transform, With<Player>>,
        input: Res<Input>,
        assets: Res<Assets>,
        mut physics: ResMut<PhysicsWorld>,
        mut commands: Commands,
    ) {
        if input.space_just_pressed {
            let player_transform = player.single();
            let pos = player_transform.position() + player_transform.forward().xyz() * 5.0;
            commands.spawn(Self::components(pos, &device, &mut physics, &assets));
        }
    }

    fn components(
        pos: Vec3,
        device: &Device,
        physics: &mut PhysicsWorld,
        assets: &Assets,
    ) -> (FreeBox, PhysicsBody, Mesh, Material, Transform, RenderTags) {
        // TODO Load in assets
        let mesh = Mesh(pollster::block_on(async {
            assets::Mesh::from_file("cube.obj", device).await
        }));
        let material = Material::diffuse(device, assets, &assets.stone_tex);
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
        let transform = Transform::new(pos, scale);

        (
            FreeBox,
            body,
            mesh,
            material,
            transform,
            RenderTags(RENDER_TAG_SCENE),
        )
    }
}

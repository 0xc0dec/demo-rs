use bevy_ecs::prelude::*;

use crate::assets;
use crate::components::{Material, Mesh, PhysicsBody, PhysicsBodyParams, RenderTags, Transform};
use crate::components::render_tags::RENDER_TAG_SCENE;
use crate::math::Vec3;
use crate::resources::{Assets, Device, PhysicsWorld};

#[derive(Component)]
pub struct FloorBox;

impl FloorBox {
    pub fn spawn(
        mut commands: Commands,
        device: Res<Device>,
        mut physics: ResMut<PhysicsWorld>,
        assets: Res<Assets>,
    ) {
        // TODO Load in assets
        let mesh = Mesh(pollster::block_on(assets::Mesh::from_file(
            "cube.obj", &device,
        )));
        let material = Material::diffuse(&device, &assets, &assets.stone_tex);
        let pos = Vec3::from_element(0.0);
        let scale = Vec3::new(10.0, 0.5, 10.0);
        let transform = Transform::new(pos, scale);
        let body = PhysicsBody::new(
            PhysicsBodyParams {
                pos,
                scale,
                rotation_axis: Vec3::from_element(0.0),
                rotation_angle: 0.0,
                movable: false,
            },
            &mut physics,
        );

        commands.spawn((
            FloorBox,
            body,
            mesh,
            material,
            transform,
            RenderTags(RENDER_TAG_SCENE),
        ));
    }
}

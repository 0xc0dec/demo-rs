use hecs::{Entity, World};

use crate::new::{assets, Assets, Device, Material, PhysicsWorld, RENDER_TAG_DEFAULT, RenderTags, Transform, Vec3};
use crate::new::physics_body::{PhysicsBody, PhysicsBodyParams};

pub struct FloorBox;

impl FloorBox {
    pub fn spawn(
        device: &Device,
        physics: &mut PhysicsWorld,
        assets: &Assets,
        world: &mut World,
    ) -> Entity {
        // TODO Load in assets
        let mesh = pollster::block_on(async { assets::Mesh::from_file("cube.obj", device).await });
        let material = Material::diffuse(device, assets, &assets.stone_tex);
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
            physics,
        );

        world.spawn((
            transform,
            mesh,
            material,
            body,
            RenderTags(RENDER_TAG_DEFAULT),
        ))
    }
}

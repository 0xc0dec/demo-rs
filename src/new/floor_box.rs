use crate::new::{assets, Assets, Device, Material, Mesh, PhysicsWorld, Transform, Vec3};
use crate::new::physics_body::{PhysicsBody, PhysicsBodyParams};

pub struct FloorBox;

impl FloorBox {
    pub fn spawn(
        device: &Device,
        physics: &mut PhysicsWorld,
        assets: &Assets,
    ) -> (PhysicsBody, Mesh, Material, Transform) {
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

        (body, mesh, material, transform)
    }
}

use hecs::World;
use rapier3d::prelude::*;

use crate::new::{PhysicsWorld, Quat, Transform, Vec3};

pub struct PhysicsBody {
    handle: RigidBodyHandle,
    movable: bool,
}

pub struct PhysicsBodyParams {
    pub pos: Vec3,
    pub scale: Vec3,
    pub rotation_angle: f32,
    pub rotation_axis: Vec3,
    pub movable: bool,
}

impl PhysicsBody {
    pub fn new(params: PhysicsBodyParams, physics: &mut PhysicsWorld) -> Self {
        let PhysicsBodyParams {
            pos,
            scale,
            rotation_axis,
            rotation_angle,
            movable,
        } = params;

        let body = RigidBodyBuilder::new(orig_type(movable))
            .translation(vector![pos.x, pos.y, pos.z])
            .rotation(rotation_axis * rotation_angle)
            .build();

        // TODO Other shapes
        let collider = ColliderBuilder::cuboid(scale.x, scale.y, scale.z)
            .restitution(0.2)
            .friction(0.7)
            .build();
        let (handle, _) = physics.add_body(body, collider);

        Self { handle, movable }
    }

    pub fn sync_to_transforms(physics: &PhysicsWorld, world: &mut World) {
        for (_, (body, tr)) in world.query::<(&PhysicsBody, &mut Transform)>().iter() {
            let (pos, rot) = body.transform(physics);
            tr.set(pos, rot);
        }
    }

    fn transform(&self, physics: &PhysicsWorld) -> (Vec3, Quat) {
        let body = physics.bodies.get(self.handle).unwrap();
        // Not sure why inverse is needed
        (*body.translation(), *body.rotation().inverse().quaternion())
    }
}

fn orig_type(movable: bool) -> RigidBodyType {
    if movable {
        RigidBodyType::Dynamic
    } else {
        RigidBodyType::Fixed
    }
}

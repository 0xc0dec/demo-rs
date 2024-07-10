use rapier3d::prelude::*;

use crate::math::Vec3;
use crate::physics_world::PhysicsWorld;

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

        let body = RigidBodyBuilder::new(body_type(movable))
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

    pub fn body_handle(&self) -> RigidBodyHandle {
        self.handle
    }

    pub fn set_kinematic(&self, physics: &mut PhysicsWorld, kinematic: bool) {
        let body = physics.bodies.get_mut(self.handle).unwrap();
        let new_type = if kinematic {
            RigidBodyType::KinematicPositionBased
        } else {
            body_type(self.movable)
        };
        body.set_body_type(new_type, true);
    }
}

fn body_type(movable: bool) -> RigidBodyType {
    if movable {
        RigidBodyType::Dynamic
    } else {
        RigidBodyType::Fixed
    }
}

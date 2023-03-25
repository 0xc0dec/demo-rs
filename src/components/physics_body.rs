use bevy_ecs::prelude::*;
use rapier3d::prelude::*;
use rapier3d::prelude::{RigidBodyBuilder, RigidBodyHandle};
use crate::math::Vec3;
use crate::physics_world::PhysicsWorld;

#[derive(Component)]
pub struct PhysicsBody {
    rigid_body_handle: RigidBodyHandle,
}

#[derive(Copy, Clone)]
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

        let body = if movable { RigidBodyBuilder::dynamic() } else { RigidBodyBuilder::fixed() }
            .translation(vector![pos.x, pos.y, pos.z])
            // TODO Verify this conversion
            .rotation(rotation_axis * rotation_angle)
            .build();
        // TODO Other shapes
        let collider = ColliderBuilder::cuboid(scale.x, scale.y, scale.z)
            .restitution(0.2)
            .friction(0.7)
            .build();
        let (rigid_body_handle, _) = physics.add_body(body, collider);

        Self {
            rigid_body_handle
        }
    }
}

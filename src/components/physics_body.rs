use crate::components::Transform;
use crate::math::Vec3;
use crate::physics_world::PhysicsWorld;
use bevy_ecs::prelude::*;
use rapier3d::prelude::*;

#[derive(Component)]
pub struct PhysicsBody {
    body_handle: RigidBodyHandle,
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

        let body = if movable {
            RigidBodyBuilder::dynamic()
        } else {
            RigidBodyBuilder::fixed()
        }
            .translation(vector![pos.x, pos.y, pos.z])
            .rotation(rotation_axis * rotation_angle)
            .build();

        // TODO Other shapes
        let collider = ColliderBuilder::cuboid(scale.x, scale.y, scale.z)
            .restitution(0.2)
            .friction(0.7)
            .build();
        let (body_handle, _) = physics.add_body(body, collider);

        Self {
            body_handle,
        }
    }

    pub fn sync(mut q: Query<(&mut Transform, &PhysicsBody)>, physics: Res<PhysicsWorld>) {
        for (mut transform, body) in q.iter_mut() {
            let body = physics.bodies.get(body.body_handle).unwrap();
            let phys_pos = body.translation();
            let phys_rot = body.rotation().inverse(); // Not sure why inverse is needed
            transform.set(*phys_pos, *phys_rot.quaternion());
        }
    }

    pub fn body_handle(&self) -> RigidBodyHandle {
        self.body_handle
    }

    pub fn grab(&self, physics: &mut PhysicsWorld) {
        let body = physics.bodies.get_mut(self.body_handle).unwrap();
        body.set_body_type(RigidBodyType::KinematicPositionBased, true);
    }

    pub fn move_to(&self, pos: Vec3, physics: &mut PhysicsWorld) {
        let body = physics.bodies.get_mut(self.body_handle).unwrap();
        body.set_translation(pos, true);
    }

    pub fn release(&self, physics: &mut PhysicsWorld) {
        let body = physics.bodies.get_mut(self.body_handle).unwrap();
        body.set_body_type(RigidBodyType::Dynamic, true);
    }
}

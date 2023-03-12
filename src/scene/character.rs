use cgmath::{Vector3};
use rapier3d::control::KinematicCharacterController;
use rapier3d::prelude::*;
use crate::frame_context::FrameContext;
use crate::physics::PhysicsWorld;
use crate::scene::spectator::Spectator;

pub struct Character {
    collider_handle: ColliderHandle,
    controller: KinematicCharacterController,
    pub spectator: Spectator,
}

impl Character {
    pub fn new(pos: Vector3<f32>, spectator: Spectator, physics: &mut PhysicsWorld) -> Self {
        let collider = ColliderBuilder::ball(0.5)
            .restitution(0.7)
            .translation(Vector::new(pos.x, pos.y, pos.z))
            .build();
        let collider_handle = physics.colliders_mut().insert(collider);
        let controller = KinematicCharacterController::default();

        Self {
            spectator,
            collider_handle,
            controller
        }
    }

    pub fn update(&mut self, ctx: &FrameContext, physics: &mut PhysicsWorld) {
        let (effective_movement, current_pos) = {
            let (current_pos, shape) = {
                let collider = physics.colliders()
                    .get(self.collider_handle)
                    .unwrap();
                (collider.position(), collider.shape())
            };

            let desired_movement = self.spectator.update(ctx);

            let effective_movement = self.controller.move_shape(
                ctx.dt,
                physics.rigid_bodies(),
                physics.colliders(),
                physics.query_pipeline(),
                shape,
                &current_pos,
                Vector::new(
                    desired_movement.x,
                    desired_movement.y,
                    desired_movement.z
                ),
                QueryFilter::default()
                    .exclude_collider(self.collider_handle),
                |_| {}
            );

            (effective_movement, current_pos.translation.vector)
        };

        self.spectator.camera.transform.translate(Vector3::new(
            effective_movement.translation.x,
            effective_movement.translation.y,
            effective_movement.translation.z
        ));

        physics.colliders_mut()
            .get_mut(self.collider_handle)
            .unwrap()
            .set_translation(current_pos + effective_movement.translation);
    }
}

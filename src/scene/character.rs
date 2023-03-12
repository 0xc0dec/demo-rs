use cgmath::{Vector3};
use rapier3d::control::KinematicCharacterController;
use rapier3d::prelude::*;
use crate::camera::Camera;
use crate::frame_context::FrameContext;
use crate::physics::PhysicsWorld;
use crate::transform::TransformSpace;

pub struct Character {
    collider_handle: ColliderHandle,
    controller: KinematicCharacterController,
    pub camera: Camera,
}

impl Character {
    pub fn new(camera: Camera, physics: &mut PhysicsWorld) -> Self {
        let cam_pos = camera.transform.position();
        let collider = ColliderBuilder::ball(0.5)
            .restitution(0.7)
            .translation(Vector::new(cam_pos.x, cam_pos.y, cam_pos.z))
            .build();
        let collider_handle = physics.colliders_mut().insert(collider);
        let controller = KinematicCharacterController::default();

        Self {
            collider_handle,
            controller,
            camera
        }
    }

    pub fn update(&mut self, ctx: &FrameContext, physics: &mut PhysicsWorld) {
        let spectator_rot = self.camera.transform.spectator_rotation(ctx.dt, ctx.events);
        if let Some(spectator_rot) = spectator_rot {
            self.camera.transform.rotate_around_axis(
                Vector3::unit_y(),
                spectator_rot.horizontal_rotation,
                TransformSpace::World
            );
            self.camera.transform.rotate_around_axis(
                Vector3::unit_x(),
                spectator_rot.vertical_rotation,
                TransformSpace::Local
            );
        }

        let spectator_translation = self.camera.transform.spectator_translation(ctx.dt, ctx.events);
        if let Some(spectator_translation) = spectator_translation {
            let (effective_movement, current_pos) = {
                let (collider_pos, collider_shape) = {
                    let collider = physics.colliders()
                        .get(self.collider_handle)
                        .unwrap();
                    (collider.position(), collider.shape())
                };

                let effective_movement = self.controller.move_shape(
                    ctx.dt,
                    physics.rigid_bodies(),
                    physics.colliders(),
                    physics.query_pipeline(),
                    collider_shape,
                    &collider_pos,
                    Vector::new(
                        spectator_translation.x,
                        spectator_translation.y,
                        spectator_translation.z
                    ),
                    QueryFilter::default()
                        .exclude_collider(self.collider_handle),
                    |_| {}
                );

                (effective_movement, collider_pos.translation.vector)
            };

            self.camera.transform.translate(Vector3::new(
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
}

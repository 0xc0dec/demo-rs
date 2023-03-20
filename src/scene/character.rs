use crate::camera::Camera;
use crate::frame_context::FrameContext;
use crate::math::{Vec3};
use crate::physics_world::PhysicsWorld;
use crate::transform::TransformSpace;
use rapier3d::prelude::*;

pub struct Character {
    pub collider_handle: ColliderHandle,
    pub camera: Camera,
}

impl Character {
    pub fn new(camera: Camera, physics: &mut PhysicsWorld) -> Self {
        let cam_pos = camera.transform.position();
        let collider = ColliderBuilder::ball(0.5)
            .restitution(0.7)
            .translation(cam_pos)
            .build();
        let collider_handle = physics.colliders.insert(collider);

        Self {
            collider_handle,
            camera,
        }
    }

    pub fn update(&mut self, ctx: &FrameContext, physics: &mut PhysicsWorld) {
        let spectator_rot = self.camera.transform.spectator_rotation(ctx.dt, &ctx.app.input);
        if let Some(spectator_rot) = spectator_rot {
            self.camera.transform.rotate_around_axis(
                Vec3::y_axis().xyz(),
                spectator_rot.horizontal_rotation,
                TransformSpace::World,
            );
            self.camera.transform.rotate_around_axis(
                Vec3::x_axis().xyz(),
                spectator_rot.vertical_rotation,
                TransformSpace::Local,
            );
        }

        let spectator_translation = self
            .camera
            .transform
            .spectator_translation(ctx.dt, 10.0, &ctx.app.input);
        if let Some(spectator_translation) = spectator_translation {
            let (effective_movement, collider_current_pos) =
                physics.move_character(ctx.dt, spectator_translation, self.collider_handle);

            self.camera.transform.translate(effective_movement);

            physics
                .colliders
                .get_mut(self.collider_handle)
                .unwrap()
                .set_translation(collider_current_pos + effective_movement);
        }
    }
}

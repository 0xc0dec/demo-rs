use crate::camera::Camera;
use crate::frame_context::FrameContext;
use crate::math::to_na_vec3;
use crate::physics_world::PhysicsWorld;
use crate::transform::TransformSpace;
use cgmath::Vector3;
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
            .translation(to_na_vec3(cam_pos))
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
                Vector3::unit_y(),
                spectator_rot.horizontal_rotation,
                TransformSpace::World,
            );
            self.camera.transform.rotate_around_axis(
                Vector3::unit_x(),
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
                .set_translation(to_na_vec3(collider_current_pos + effective_movement));
        }

        // Testing raycasting - TODO remove
        physics.cast_ray(
            self.camera.transform.position(),
            -self.camera.transform.forward(), // For some reason the ray is inverted here :(
            Some(self.collider_handle),
        );
    }
}

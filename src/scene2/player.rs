use bevy_ecs::prelude::{Component, NonSend, Query, Res};
use bevy_ecs::system::NonSendMut;
use crate::camera::Camera;
use crate::math::{Vec3};
use crate::physics_world::PhysicsWorld;
use crate::transform::TransformSpace;
use rapier3d::prelude::*;
use crate::input::Input;
use crate::state::State;

// TODO Split into multiple components
#[derive(Component)]
pub struct Player {
    pub collider_handle: ColliderHandle,
    pub camera: Camera, // TODO Split into two components
}

impl Player {
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

    pub fn update(
        mut q: Query<&mut Player>,
        state: Res<State>,
        mut physics: NonSendMut<PhysicsWorld>,
        input: NonSend<Input>
    ) {
        let dt = state.frame_time.delta;

        let mut player = q.iter_mut().next().unwrap();

        let spectator_rot = player.camera.transform
            .spectator_rotation(dt, &input);
        if let Some(spectator_rot) = spectator_rot {
            player.camera.transform.rotate_around_axis(
                Vec3::y_axis().xyz(),
                spectator_rot.horizontal_rotation,
                TransformSpace::World,
            );
            player.camera.transform.rotate_around_axis(
                Vec3::x_axis().xyz(),
                spectator_rot.vertical_rotation,
                TransformSpace::Local,
            );
        }

        let spectator_translation = player.camera.transform
            .spectator_translation(dt, 10.0, &input);
        if let Some(spectator_translation) = spectator_translation {
            let (effective_movement, collider_current_pos) =
                physics.move_character(dt, spectator_translation, player.collider_handle);

            player.camera.transform.translate(effective_movement);

            physics
                .colliders
                .get_mut(player.collider_handle)
                .unwrap()
                .set_translation(collider_current_pos + effective_movement);
        }
    }
}

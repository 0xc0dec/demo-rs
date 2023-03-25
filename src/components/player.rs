use bevy_ecs::prelude::{Commands, Component, NonSend, Query, Res};
use bevy_ecs::system::NonSendMut;
use crate::components::camera::Camera;
use crate::math::{Vec3};
use crate::physics_world::PhysicsWorld;
use crate::transform::TransformSpace;
use rapier3d::prelude::*;
use crate::device::Device;
use crate::input_state::InputState;
use crate::state::State;

#[derive(Component)]
pub struct Player {
    pub collider_handle: ColliderHandle,
}

impl Player {
    pub fn spawn(
        device: NonSend<Device>,
        mut physics: NonSendMut<PhysicsWorld>,
        mut commands: Commands,
    ) {
        let canvas_size: (f32, f32) = device.surface_size().into();
        let camera = Camera::new(
            Vec3::new(10.0, 10.0, 10.0),
            Vec3::new(0.0, 0.0, 0.0),
            canvas_size,
        );

        commands.spawn((
            Self::new(&camera, &mut physics),
            camera,
        ));
    }

    pub fn update(
        mut q: Query<(&mut Self, &mut Camera)>,
        state: Res<State>,
        mut physics: NonSendMut<PhysicsWorld>,
        input: Res<InputState>,
    ) {
        // TODO Update camera FOV
        // self.character.camera.set_fov(
        //     ctx.app.device.surface_size().width as f32,
        //     ctx.app.device.surface_size().height as f32,
        // );

        let dt = state.frame_time.delta;

        let (player, mut camera) = q.iter_mut().next().unwrap();

        let spectator_rot = camera.transform
            .spectator_rotation(dt, &input);
        if let Some(spectator_rot) = spectator_rot {
            camera.transform.rotate_around_axis(
                Vec3::y_axis().xyz(),
                spectator_rot.horizontal_rotation,
                TransformSpace::World,
            );
            camera.transform.rotate_around_axis(
                Vec3::x_axis().xyz(),
                spectator_rot.vertical_rotation,
                TransformSpace::Local,
            );
        }

        let spectator_translation = camera.transform
            .spectator_translation(dt, 10.0, &input);
        if let Some(spectator_translation) = spectator_translation {
            let (effective_movement, collider_current_pos) =
                physics.move_character(dt, spectator_translation, player.collider_handle);

            camera.transform.translate(effective_movement);

            physics
                .colliders
                .get_mut(player.collider_handle)
                .unwrap()
                .set_translation(collider_current_pos + effective_movement);
        }
    }

    fn new(camera: &Camera, physics: &mut PhysicsWorld) -> Self {
        let cam_pos = camera.transform.position();
        let collider = ColliderBuilder::ball(0.5)
            .restitution(0.7)
            .translation(cam_pos)
            .build();
        let collider_handle = physics.colliders.insert(collider);

        Self {
            collider_handle,
        }
    }
}

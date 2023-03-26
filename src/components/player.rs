use bevy_ecs::prelude::{Commands, Component, EventReader, NonSend, Query, Res};
use bevy_ecs::system::NonSendMut;
use crate::components::camera::Camera;
use crate::math::{Vec3};
use crate::physics_world::PhysicsWorld;
use crate::components::transform::TransformSpace;
use rapier3d::prelude::*;
use crate::components::Transform;
use crate::device::Device;
use crate::events::WindowResizeEvent;
use crate::input_state::InputState;
use crate::render_tags::RenderTags;
use crate::state::State;

#[derive(Component)]
pub struct Player {
    collider_handle: ColliderHandle,
}

impl Player {
    pub fn spawn(
        device: NonSend<Device>,
        mut physics: NonSendMut<PhysicsWorld>,
        mut commands: Commands,
    ) {
        let pos = Vec3::new(10.0, 10.0, 10.0);

        let camera = Camera::new(
            device.surface_size().width as f32 / device.surface_size().height as f32,
            RenderTags::SCENE,
            None
        );
        let mut transform = Transform::from_pos(pos);
        transform.look_at(Vec3::from_element(0.0));

        let collider = ColliderBuilder::ball(0.5)
            .restitution(0.7)
            .translation(pos)
            .build();
        let collider_handle = physics.colliders.insert(collider);

        commands.spawn((
            Player { collider_handle },
            camera,
            transform
        ));
    }

    pub fn update(
        state: Res<State>,
        input: Res<InputState>,
        mut q: Query<(&mut Self, &mut Camera, &mut Transform)>,
        mut physics: NonSendMut<PhysicsWorld>,
        mut resize_events: EventReader<WindowResizeEvent>
    ) {
        let dt = state.frame_time.delta;

        let (player, mut camera, mut transform) = q.single_mut();

        for e in resize_events.iter() {
            camera.set_aspect(e.new_size.width as f32 / e.new_size.height as f32);
        }

        let spectator_rot = transform
            .spectator_rotation(dt, &input);
        if let Some(spectator_rot) = spectator_rot {
            transform.rotate_around_axis(
                Vec3::y_axis().xyz(),
                spectator_rot.horizontal_rotation,
                TransformSpace::World,
            );
            transform.rotate_around_axis(
                Vec3::x_axis().xyz(),
                spectator_rot.vertical_rotation,
                TransformSpace::Local,
            );
        }

        let spectator_translation = transform
            .spectator_translation(dt, 10.0, &input);
        if let Some(spectator_translation) = spectator_translation {
            let (effective_movement, collider_current_pos) =
                physics.move_character(dt, spectator_translation, player.collider_handle);

            transform.translate(effective_movement);

            physics
                .colliders
                .get_mut(player.collider_handle)
                .unwrap()
                .set_translation(collider_current_pos + effective_movement);
        }
    }
}

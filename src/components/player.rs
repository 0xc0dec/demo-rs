use std::f32::consts::PI;
use crate::components::camera::Camera;
use crate::components::transform::TransformSpace;
use crate::components::Transform;
use crate::device::Device;
use crate::events::WindowResizeEvent;
use crate::input::Input;
use crate::math::Vec3;
use crate::physics_world::PhysicsWorld;
use crate::render_tags::RenderTags;
use crate::render_target::RenderTarget;
use bevy_ecs::prelude::*;
use rapier3d::prelude::*;
use crate::frame_time::FrameTime;

#[derive(Component)]
pub struct Player {
    collider_handle: ColliderHandle,
}

impl Player {
    pub fn spawn(
        device: Res<Device>,
        mut physics: ResMut<PhysicsWorld>,
        mut commands: Commands,
    ) {
        let pos = Vec3::new(10.0, 10.0, 10.0);

        let rt = RenderTarget::new(&device, None);
        let camera = Camera::new(
            device.surface_size().width as f32 / device.surface_size().height as f32,
            RenderTags::SCENE,
            Some(rt),
        );
        let mut transform = Transform::from_pos(pos);
        transform.look_at(Vec3::from_element(0.0));

        let collider = ColliderBuilder::ball(0.5)
            .restitution(0.7)
            .translation(pos)
            .build();
        let collider_handle = physics.colliders.insert(collider);

        // TODO Use component bundles?
        commands.spawn((Player { collider_handle }, camera, transform));
    }

    pub fn collider_handle(&self) -> ColliderHandle {
        self.collider_handle
    }

    pub fn update(
        frame_time: Res<FrameTime>,
        device: Res<Device>,
        input: Res<Input>,
        mut q: Query<(&mut Self, &mut Camera, &mut Transform)>,
        mut physics: ResMut<PhysicsWorld>,
        mut resize_events: EventReader<WindowResizeEvent>,
    ) {
        let (player, mut camera, mut transform) = q.single_mut();

        let last_resize = resize_events.iter().last();
        if let Some(last_resize) = last_resize {
            camera.set_aspect(
                last_resize.new_size.width as f32 / last_resize.new_size.height as f32
            );
            if let Some(target) = camera.target_mut() {
                target.resize(
                    (last_resize.new_size.width, last_resize.new_size.height),
                    &device
                )
            }
        }

        let dt = frame_time.delta;

        if input.rmb_down {
            Self::rotate(&mut transform, dt, &input);
            Self::translate(&mut transform, player.collider_handle, dt, 10.0, &input, &mut physics);
        }
    }

    fn rotate(
        transform: &mut Transform,
        dt: f32,
        input: &Input,
    ) {
        const MIN_TOP_ANGLE: f32 = 0.1;
        const MIN_BOTTOM_ANGLE: f32 = PI - 0.1;
        let angle_to_top = transform.forward().angle(&Vec3::y_axis());
        let mut v_rot = input.mouse_delta.1 * dt;
        // Protect from overturning - prevent camera from reaching the vertical line with small
        // margin angles.
        if angle_to_top + v_rot <= MIN_TOP_ANGLE {
            v_rot = -(angle_to_top - MIN_TOP_ANGLE);
        } else if angle_to_top + v_rot >= MIN_BOTTOM_ANGLE {
            v_rot = MIN_BOTTOM_ANGLE - angle_to_top;
        }

        let h_rot = input.mouse_delta.0 * dt;

        transform.rotate_around_axis(Vec3::y_axis().xyz(), h_rot, TransformSpace::World);
        transform.rotate_around_axis(Vec3::x_axis().xyz(), v_rot, TransformSpace::Local);
    }

    fn translate(
        transform: &mut Transform,
        collider_handle: ColliderHandle,
        dt: f32,
        speed: f32,
        input: &Input,
        physics: &mut PhysicsWorld
    ) {
        let mut translation: Vec3 = Vec3::from_element(0.0);
        if input.forward_down {
            translation += transform.forward();
        }
        if input.back_down {
            translation -= transform.forward();
        }
        if input.right_down {
            translation += transform.right();
        }
        if input.left_down {
            translation -= transform.right();
        }
        if input.up_down {
            translation += transform.up();
        }
        if input.down_down {
            translation -= transform.up();
        }
        let translation = translation.normalize() * dt * speed;

        let (effective_movement, collider_current_pos) =
            physics.move_character(dt, translation, collider_handle);

        transform.translate(effective_movement);

        physics
            .colliders
            .get_mut(collider_handle)
            .unwrap()
            .set_translation(collider_current_pos + effective_movement);

    }
}

use std::f32::consts::PI;

use bevy_ecs::prelude::*;
use rapier3d::prelude::*;
use winit::window::{CursorGrabMode, Window};

use crate::assets::RenderTarget;
use crate::components::*;
use crate::events::ResizeEvent;
use crate::math::Vec3;
use crate::resources::{Device, FrameTime, Input, InputAction, PhysicsWorld};

#[derive(Component)]
pub struct Player {
    target_pt: Option<Vec3>,
    target_body: Option<RigidBodyHandle>,
    collider_handle: ColliderHandle,
    h_rot_acc: f32,
    v_rot_acc: f32,
    translation_acc: Vec3,
    controlled: bool,
}

impl Player {
    pub fn spawn(device: Res<Device>, mut physics: ResMut<PhysicsWorld>, mut commands: Commands) {
        let pos = Vec3::new(7.0, 7.0, 7.0);

        let rt = RenderTarget::new(&device, None);
        let camera = Camera::new(
            device.surface_size().width as f32 / device.surface_size().height as f32,
            RENDER_TAG_SCENE,
            Some(rt),
        );
        let mut transform = Transform::from_pos(pos);
        transform.look_at(Vec3::from_element(0.0));

        let collider = ColliderBuilder::ball(0.5)
            .restitution(0.7)
            .translation(pos)
            .build();
        let collider_handle = physics.colliders.insert(collider);

        commands.spawn((
            Player {
                collider_handle,
                target_pt: None,
                target_body: None,
                h_rot_acc: 0.0,
                v_rot_acc: 0.0,
                translation_acc: Vec3::zeros(),
                controlled: false,
            },
            camera,
            transform,
        ));
    }

    pub fn target_pt(&self) -> Option<Vec3> {
        self.target_pt
    }

    pub fn target_body(&self) -> Option<RigidBodyHandle> {
        self.target_body
    }

    pub fn update(
        frame_time: Res<FrameTime>,
        device: Res<Device>,
        input: Res<Input>,
        window: NonSend<Window>,
        mut player: Query<(&mut Self, &mut Camera, &mut Transform)>,
        mut physics: ResMut<PhysicsWorld>,
        mut events: EventReader<ResizeEvent>,
    ) {
        let (mut player, mut cam, mut tr) = player.single_mut();

        // Update camera aspect and RT size
        if let Some(e) = events.read().last() {
            cam.set_aspect(e.0.width as f32 / e.0.height as f32);
            if let Some(target) = cam.target_mut() {
                target.resize((e.0.width, e.0.height), &device);
            }
        }

        // Move and rotate
        let dt = frame_time.delta;
        if player.controlled {
            player.rotate(&mut tr, &input, dt);
            player.translate(&mut tr, dt, &input, &mut physics);
        }

        if input.action_activated(InputAction::ControlPlayer) {
            player.controlled = !player.controlled;
            toggle_mouse_grab(player.controlled, &window);
        }

        update_target((&mut player, &tr), &physics);
    }

    fn translate(
        &mut self,
        transform: &mut Transform,
        dt: f32,
        input: &Input,
        physics: &mut PhysicsWorld,
    ) {
        let mut translation: Vec3 = Vec3::from_element(0.0);

        if input.action_active(InputAction::MoveForward) {
            translation += transform.forward();
        }
        if input.action_active(InputAction::MoveBack) {
            translation -= transform.forward();
        }
        if input.action_active(InputAction::MoveRight) {
            translation += transform.right();
        }
        if input.action_active(InputAction::MoveLeft) {
            translation -= transform.right();
        }
        if input.action_active(InputAction::MoveUp) {
            translation += transform.up();
        }
        if input.action_active(InputAction::MoveDown) {
            translation -= transform.up();
        }

        const SPEED: f32 = 10.0;

        // Apply only if there's anything to apply. Otherwise getting NaN after normalize() :|
        if translation.magnitude() > 0.01 {
            self.translation_acc += translation.normalize() * dt * SPEED;
        }

        let (possible_translation, collider_current_pos) =
            physics.move_character(dt, self.translation_acc, self.collider_handle);
        self.translation_acc = possible_translation;

        let translation = SPEED * dt * self.translation_acc;
        self.translation_acc -= translation;

        transform.translate(translation);
        physics
            .colliders
            .get_mut(self.collider_handle)
            .unwrap()
            .set_translation(collider_current_pos + translation);
    }

    fn rotate(&mut self, transform: &mut Transform, input: &Input, dt: f32) {
        const MIN_TOP_ANGLE: f32 = 0.1;
        const MIN_BOTTOM_ANGLE: f32 = PI - 0.1;
        const SPEED: f32 = 25.0;

        let angle_to_top = transform.forward().angle(&Vec3::y_axis());
        self.v_rot_acc += input.mouse_delta.1 * dt;
        // Protect from overturning - prevent camera from reaching the vertical line with small
        // margin angles.
        if angle_to_top + self.v_rot_acc <= MIN_TOP_ANGLE {
            self.v_rot_acc = -(angle_to_top - MIN_TOP_ANGLE);
        } else if angle_to_top + self.v_rot_acc >= MIN_BOTTOM_ANGLE {
            self.v_rot_acc = MIN_BOTTOM_ANGLE - angle_to_top;
        }

        // Smooth the movement a bit
        let v_rot = SPEED * dt * self.v_rot_acc;
        self.v_rot_acc -= v_rot;

        self.h_rot_acc += input.mouse_delta.0 * dt;
        let h_rot = SPEED * dt * self.h_rot_acc;
        self.h_rot_acc -= h_rot;

        transform.rotate_around_axis(Vec3::y_axis().xyz(), h_rot, TransformSpace::World);
        transform.rotate_around_axis(Vec3::x_axis().xyz(), v_rot, TransformSpace::Local);
    }
}

fn toggle_mouse_grab(grab: bool, window: &Window) {
    if grab {
        window
            .set_cursor_grab(CursorGrabMode::Confined)
            .or_else(|_e| window.set_cursor_grab(CursorGrabMode::Locked))
            .unwrap();
        window.set_cursor_visible(false);
    } else {
        window.set_cursor_grab(CursorGrabMode::None).unwrap();
        window.set_cursor_visible(true);
    }
}

fn update_target(player: (&mut Player, &Transform), physics: &PhysicsWorld) {
    if let Some((hit_pt, _, hit_collider)) = physics.cast_ray(
        player.1.position(),
        player.1.forward(),
        Some(player.0.collider_handle),
    ) {
        player.0.target_pt = Some(hit_pt);
        player.0.target_body = Some(
            physics
                .colliders
                .get(hit_collider)
                .unwrap()
                .parent()
                .unwrap(),
        );
    } else {
        player.0.target_pt = None;
        player.0.target_body = None;
    }
}

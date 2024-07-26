use std::f32::consts::PI;

use hecs::{Entity, World};
use rapier3d::prelude::*;
use winit::window::{CursorGrabMode, Window};

use crate::camera::Camera;
use crate::graphics::{Graphics, SurfaceSize};
use crate::input::{Input, InputAction};
use crate::math::Vec3;
use crate::physics::Physics;
use crate::render_tags::RENDER_TAG_SCENE;
use crate::render_target::RenderTarget;
use crate::transform::{Transform, TransformSpace};

pub struct Player {
    // Point and physics body at which the player is currently looking at
    // (ray cast from the screen center).
    focus_pt: Option<Vec3>,
    focus_body: Option<RigidBodyHandle>,
    // TODO Extract into a component
    collider_handle: ColliderHandle,
    h_rot_acc: f32,
    v_rot_acc: f32,
    translation_acc: Vec3,
    controlled: bool,
}

impl Player {
    pub fn spawn(w: &mut World, gfx: &Graphics, physics: &mut Physics) -> Entity {
        let pos = Vec3::new(7.0, 7.0, 7.0);

        let rt = RenderTarget::new(gfx, None);
        let camera = Camera::new(
            gfx.surface_size().width as f32 / gfx.surface_size().height as f32,
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

        w.spawn((
            Self {
                collider_handle,
                focus_pt: None,
                focus_body: None,
                h_rot_acc: 0.0,
                v_rot_acc: 0.0,
                translation_acc: Vec3::zeros(),
                controlled: false,
            },
            camera,
            transform,
        ))
    }

    pub fn focus_point(&self) -> Option<Vec3> {
        self.focus_pt
    }

    pub fn focus_body(&self) -> Option<RigidBodyHandle> {
        self.focus_body
    }

    pub fn controlled(&self) -> bool {
        self.controlled
    }

    pub fn update(
        world: &mut World,
        physics: &mut Physics,
        gfx: &Graphics,
        input: &Input,
        window: &Window,
        dt: f32,
        new_canvas_size: &Option<SurfaceSize>,
    ) {
        let mut q = world.query::<(&mut Camera, &mut Transform, &mut Player)>();
        let (_, (cam, tr, player)) = q.iter().next().unwrap();

        // TODO Extract this out of Player
        // Update camera aspect and RT size
        if let Some(&size) = new_canvas_size.as_ref() {
            cam.set_aspect(size.width as f32 / size.height as f32);
            if let Some(target) = cam.target_mut() {
                target.resize((size.width, size.height), gfx);
            }
        }

        // Move and rotate
        if player.controlled {
            player.rotate(dt, tr, input);
            player.translate(dt, tr, input, physics);
        } else {
            player.translation_acc = Vec3::zeros();
        }

        if input.action_activated(InputAction::ControlPlayer) {
            player.controlled = !player.controlled;
            toggle_cursor(player.controlled, window);
        }

        player.update_focus(tr, physics);
    }

    // TODO Extract this + rotate() into a separate component?
    fn translate(
        &mut self,
        dt: f32,
        transform: &mut Transform,
        input: &Input,
        physics: &mut Physics,
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

    fn rotate(&mut self, dt: f32, transform: &mut Transform, input: &Input) {
        const MIN_TOP_ANGLE: f32 = 0.1;
        const MIN_BOTTOM_ANGLE: f32 = PI - 0.1;
        const SPEED: f32 = 25.0;

        let angle_to_top = transform.forward().angle(&Vec3::y_axis());
        self.v_rot_acc += input.mouse_delta().1 * dt;
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

        self.h_rot_acc += input.mouse_delta().0 * dt;
        let h_rot = SPEED * dt * self.h_rot_acc;
        self.h_rot_acc -= h_rot;

        transform.rotate_around_axis(Vec3::y_axis().xyz(), h_rot, TransformSpace::World);
        transform.rotate_around_axis(Vec3::x_axis().xyz(), v_rot, TransformSpace::Local);
    }

    fn update_focus(&mut self, transform: &Transform, physics: &Physics) {
        if let Some((hit_pt, _, hit_collider)) = physics.cast_ray(
            transform.position(),
            transform.forward(),
            Some(self.collider_handle),
        ) {
            self.focus_pt = Some(hit_pt);
            self.focus_body = Some(
                physics
                    .colliders
                    .get(hit_collider)
                    .unwrap()
                    .parent()
                    .unwrap(),
            );
        } else {
            self.focus_pt = None;
            self.focus_body = None;
        }
    }
}

fn toggle_cursor(grab: bool, window: &Window) {
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

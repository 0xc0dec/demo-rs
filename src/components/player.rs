use std::f32::consts::PI;

use rapier3d::prelude::*;
use winit::window::{CursorGrabMode, Window};

use crate::assets::RenderTarget;
use crate::components::*;
use crate::events::ResizeEvent;
use crate::frame_time::FrameTime;
use crate::graphics::Graphics;
use crate::input::{Input, InputAction};
use crate::math::Vec3;
use crate::physics::Physics;

pub struct Player {
    // Point and physics body at which the player is currently looking at
    // (ray cast from the screen center).
    focus_pt: Option<Vec3>,
    focus_body: Option<RigidBodyHandle>,
    collider_handle: ColliderHandle,
    h_rot_acc: f32,
    v_rot_acc: f32,
    translation_acc: Vec3,
    controlled: bool,
    camera: Camera,
    transform: Transform,
}

impl Player {
    pub fn new(gfx: &Graphics, physics: &mut Physics) -> Self {
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

        Self {
            collider_handle,
            focus_pt: None,
            focus_body: None,
            h_rot_acc: 0.0,
            v_rot_acc: 0.0,
            translation_acc: Vec3::zeros(),
            controlled: false,
            camera,
            transform,
        }
    }

    pub fn camera(&self) -> &Camera {
        &self.camera
    }

    pub fn transform(&self) -> &Transform {
        &self.transform
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
        &mut self,
        gfx: &Graphics,
        frame_time: &FrameTime,
        input: &Input,
        window: &Window,
        physics: &mut Physics,
        resize_event: &Option<ResizeEvent>,
    ) {
        // Update camera aspect and RT size
        if let Some(e) = resize_event {
            self.camera.set_aspect(e.0.width as f32 / e.0.height as f32);
            if let Some(target) = self.camera.target_mut() {
                target.resize((e.0.width, e.0.height), gfx);
            }
        }

        // Move and rotate
        let dt = frame_time.delta;
        if self.controlled {
            self.rotate(dt, input);
            self.translate(dt, input, physics);
        } else {
            self.translation_acc = Vec3::zeros();
        }

        if input.action_activated(InputAction::ControlPlayer) {
            self.controlled = !self.controlled;
            toggle_cursor(self.controlled, window);
        }

        self.update_focus(physics);
    }

    fn translate(&mut self, dt: f32, input: &Input, physics: &mut Physics) {
        let mut translation: Vec3 = Vec3::from_element(0.0);

        if input.action_active(InputAction::MoveForward) {
            translation += self.transform.forward();
        }
        if input.action_active(InputAction::MoveBack) {
            translation -= self.transform.forward();
        }
        if input.action_active(InputAction::MoveRight) {
            translation += self.transform.right();
        }
        if input.action_active(InputAction::MoveLeft) {
            translation -= self.transform.right();
        }
        if input.action_active(InputAction::MoveUp) {
            translation += self.transform.up();
        }
        if input.action_active(InputAction::MoveDown) {
            translation -= self.transform.up();
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

        self.transform.translate(translation);
        physics
            .colliders
            .get_mut(self.collider_handle)
            .unwrap()
            .set_translation(collider_current_pos + translation);
    }

    fn rotate(&mut self, dt: f32, input: &Input) {
        const MIN_TOP_ANGLE: f32 = 0.1;
        const MIN_BOTTOM_ANGLE: f32 = PI - 0.1;
        const SPEED: f32 = 25.0;

        let angle_to_top = self.transform.forward().angle(&Vec3::y_axis());
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

        self.transform
            .rotate_around_axis(Vec3::y_axis().xyz(), h_rot, TransformSpace::World);
        self.transform
            .rotate_around_axis(Vec3::x_axis().xyz(), v_rot, TransformSpace::Local);
    }

    fn update_focus(&mut self, physics: &Physics) {
        if let Some((hit_pt, _, hit_collider)) = physics.cast_ray(
            self.transform.position(),
            self.transform.forward(),
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

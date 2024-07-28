use std::f32::consts::PI;

use hecs::{Entity, World};
use rapier3d::prelude::*;
use winit::window::{CursorGrabMode, Window};

use crate::components::RENDER_TAG_SCENE;
use crate::graphics::Graphics;
use crate::input::{Input, InputAction};
use crate::math::{to_point3, Vec2, Vec3};
use crate::physics::Physics;
use crate::render_target::RenderTarget;

use super::camera::Camera;
use super::transform::{Transform, TransformSpace};

pub struct Player {
    // Point and physics body at which the player is currently looking at
    // (ray cast from the screen center).
    focus_at_pt: Option<Vec3>,
    focus_at_body: Option<RigidBodyHandle>,
    // TODO Extract into a component
    collider: ColliderHandle,
    h_rot_acc: f32,
    v_rot_acc: f32,
    translation_acc: Vec3,
    controlled: bool,
}

impl Player {
    pub fn spawn(w: &mut World, gfx: &Graphics, physics: &mut Physics, position: Vec3) -> Entity {
        let rt = RenderTarget::new(gfx, None);
        let camera = Camera::new(
            gfx.surface_size().width as f32 / gfx.surface_size().height as f32,
            RENDER_TAG_SCENE,
            Some(rt),
        );

        let mut transform = Transform::from_pos(position);
        transform.look_at(Vec3::from_element(0.0));

        let collider = ColliderBuilder::ball(0.5)
            .restitution(0.7)
            .translation(position)
            .build();
        let collider = physics.colliders.insert(collider);

        w.spawn((
            Self {
                collider,
                focus_at_pt: None,
                focus_at_body: None,
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
        self.focus_at_pt
    }

    pub fn focus_body(&self) -> Option<RigidBodyHandle> {
        self.focus_at_body
    }

    pub fn update(
        dt: f32,
        world: &mut World,
        physics: &mut Physics,
        input: &Input,
        window: &Window,
    ) {
        let (_, (tr, cam, player)) = world
            .query_mut::<(&mut Transform, &mut Camera, &mut Player)>()
            .into_iter()
            .next()
            .unwrap();

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

        player.update_focus(tr, cam, input, window, physics);
    }

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
            physics.move_character(dt, self.translation_acc, self.collider);
        self.translation_acc = possible_translation;

        let translation = SPEED * dt * self.translation_acc;
        self.translation_acc -= translation;

        transform.translate(translation);
        physics
            .colliders
            .get_mut(self.collider)
            .unwrap()
            .set_translation(collider_current_pos + translation);
    }

    fn rotate(&mut self, dt: f32, transform: &mut Transform, input: &Input) {
        const MIN_TOP_ANGLE: f32 = 0.1;
        const MIN_BOTTOM_ANGLE: f32 = PI - 0.1;
        const SPEED: f32 = 30.0;

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

    fn update_focus(
        &mut self,
        tr: &Transform,
        cam: &Camera,
        input: &Input,
        window: &Window,
        physics: &Physics,
    ) {
        // TODO Fix these conditions, they should rely on player being controlled
        let ray = if self.controlled {
            // From screen center
            Some((tr.position(), tr.forward()))
        } else if let Some(cursor_pos) = input.cursor_position() {
            // From cursor position
            let cursor_pos = Vec2::new(cursor_pos.0, cursor_pos.1);
            let canvas_size = Vec2::new(
                window.inner_size().width as f32,
                window.inner_size().height as f32,
            );
            // Normalized device coordinates (-1..1)
            let mut cursor_ndc_pos =
                (cursor_pos.component_div(&canvas_size)) * 2.0 - Vec2::from_element(1.0);
            // Needed for some reason... Is there a bug somewhere that gets compensated by this, or is wgpu
            // NDC origin in the lower left window corner?
            cursor_ndc_pos.y *= -1.0;
            let m = tr.matrix() * cam.proj_matrix().try_inverse().unwrap();
            let cursor_world_pos = m.transform_point(&to_point3(Vec3::new(
                cursor_ndc_pos.x,
                cursor_ndc_pos.y,
                -1.0,
            )));
            let cursor_world_pos =
                Vec3::new(cursor_world_pos.x, cursor_world_pos.y, cursor_world_pos.z);

            let orig = tr.position();
            let dir = (cursor_world_pos - orig).normalize();

            Some((tr.position(), dir))
        } else {
            None
        };

        if let Some((orig, dir)) = ray {
            if let Some((hit_pt, _, hit_collider)) =
                physics.cast_ray(orig, dir, Some(self.collider))
            {
                self.focus_at_pt = Some(hit_pt);
                self.focus_at_body = Some(
                    physics
                        .colliders
                        .get(hit_collider)
                        .unwrap()
                        .parent()
                        .unwrap(),
                );
                return;
            }
        }

        self.focus_at_pt = None;
        self.focus_at_body = None;
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

use std::f32::consts::PI;
use crate::input::Input;
use crate::math::Vec3;

pub struct SpectatorRotationDelta {
    pub vertical_rotation: f32,
    pub horizontal_rotation: f32,
}

// TODO Move out of components?
impl crate::transform::Transform {
    pub fn spectator_rotation(&self, dt: f32, input: &Input) -> Option<SpectatorRotationDelta> {
        if !input.rmb_down {
            return None;
        }

        let h_rot = input.mouse_delta.0 as f32 * dt;

        const MIN_TOP_ANGLE: f32 = 0.1;
        const MIN_BOTTOM_ANGLE: f32 = PI - 0.1;
        let angle_to_top = self.forward().angle(&Vec3::y_axis());
        let mut v_rot = input.mouse_delta.1 as f32 * dt;
        // Protect from overturning - prevent camera from reaching the vertical line with small
        // margin angles.
        if angle_to_top + v_rot <= MIN_TOP_ANGLE {
            v_rot = -(angle_to_top - MIN_TOP_ANGLE);
        } else if angle_to_top + v_rot >= MIN_BOTTOM_ANGLE {
            v_rot = MIN_BOTTOM_ANGLE - angle_to_top;
        }

        Some(SpectatorRotationDelta {
            horizontal_rotation: h_rot,
            vertical_rotation: v_rot,
        })
    }

    pub fn spectator_translation(
        &self,
        dt: f32,
        speed: f32,
        input: &Input,
    ) -> Option<Vec3> {
        if !input.rmb_down {
            return None;
        }

        let mut movement: Vec3 = Vec3::from_element(0.0);
        if input.forward_down {
            movement += self.forward();
        }
        if input.back_down {
            movement -= self.forward();
        }
        if input.right_down {
            movement += self.right();
        }
        if input.left_down {
            movement -= self.right();
        }
        if input.up_down {
            movement += self.up();
        }
        if input.down_down {
            movement -= self.up();
        }

        Some(movement.normalize() * dt * speed)
    }
}

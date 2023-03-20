use crate::input::Input;
use cgmath::*;
use crate::math::Vec3;

pub struct SpectatorRotationDelta {
    pub vertical_rotation: Rad<f32>,
    pub horizontal_rotation: Rad<f32>,
}

impl crate::transform::Transform {
    pub fn spectator_rotation(&self, dt: f32, input: &Input) -> Option<SpectatorRotationDelta> {
        if !input.rmb_down {
            return None;
        }

        let hdelta = input.mouse_delta.0 as f32 * dt;
        let horizontal_rotation = Rad(hdelta);

        let forward = self.forward();
        let angle_to_up = forward.angle(&Vec3::y_axis());
        let mut vdelta = input.mouse_delta.1 as f32 * dt;
        if vdelta < 0.0 {
            // Moving up
            if angle_to_up + vdelta <= 0.1 {
                vdelta = -(angle_to_up - 0.1);
            }
        } else if angle_to_up + vdelta >= 3.04 {
            vdelta = 3.04 - angle_to_up;
        }

        let vertical_rotation = Rad(vdelta);

        Some(SpectatorRotationDelta {
            horizontal_rotation,
            vertical_rotation,
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

        let mut movement: Vec3 = Vec3::zero();
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

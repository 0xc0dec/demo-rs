use std::f32::consts::PI;
use crate::input::Input;
use crate::math::Vec3;

pub struct SpectatorRotationDelta {
    pub vertical_rotation: f32,
    pub horizontal_rotation: f32,
}

impl crate::transform::Transform {
    pub fn spectator_rotation(&self, dt: f32, input: &Input) -> Option<SpectatorRotationDelta> {
        if !input.rmb_down {
            return None;
        }

        let h_rot = input.mouse_delta.0 as f32 * dt;

        let forward = self.forward();
        let angle_to_up = (forward.angle(&Vec3::y_axis()) / PI) * 180.0;
        let mut v_rot = input.mouse_delta.1 as f32 * dt;
        println!("{angle_to_up} {v_rot:?}");
        // TODO Fix camera jumping
        if angle_to_up + v_rot <= 10.0 {
            v_rot = 10.0 - angle_to_up;
        } else if angle_to_up + v_rot >= 170.0 {
            v_rot = 170.0 - angle_to_up;
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

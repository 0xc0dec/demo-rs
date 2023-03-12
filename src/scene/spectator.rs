use cgmath::*;
use crate::events::Events;

pub struct SpectatorRotationDelta {
    pub vertical_rotation: Rad<f32>,
    pub horizontal_rotation: Rad<f32>
}

impl crate::transform::Transform {
    pub fn spectator_rotation(&self, dt: f32, events: &Events) -> Option<SpectatorRotationDelta> {
        if !events.rmb_down { return None; }

        let hdelta = -events.mouse_delta.0 as f32 * dt;
        let horizontal_rotation = Rad(hdelta);

        let forward = self.forward();
        let angle_to_up = forward.angle(Vector3::unit_y()).0;
        let mut vdelta = -events.mouse_delta.1 as f32 * dt;
        if vdelta < 0.0 { // Moving up
            if angle_to_up + vdelta <= 0.1 {
                vdelta = -(angle_to_up - 0.1);
            }
        } else if angle_to_up + vdelta >= 3.04 {
            vdelta = 3.04 - angle_to_up;
        }

        let vertical_rotation = Rad(vdelta);

        Some(SpectatorRotationDelta {
            horizontal_rotation,
            vertical_rotation
        })
    }

    pub fn spectator_translation(&self, dt: f32, events: &Events) -> Option<Vector3<f32>> {
        if !events.rmb_down { return None; }

        let mut movement: Vector3<f32> = Vector3::zero();
        if events.forward_down {
            movement -= self.forward();
        }
        if events.back_down {
            movement += self.forward();
        }
        if events.right_down {
            movement += self.right();
        }
        if events.left_down {
            movement -= self.right();
        }
        if events.up_down {
            movement += self.up();
        }
        if events.down_down {
            movement -= self.up();
        }

        Some(movement.normalize() * dt * 10.0)
    }
}


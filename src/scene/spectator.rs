use cgmath::*;
use crate::camera::Camera;
use crate::frame_context::FrameContext;
use crate::transform::TransformSpace;

pub struct Spectator {
    pub camera: Camera,
}

impl Spectator {
    pub fn update(&mut self, frame_context: &FrameContext) {
        let FrameContext { events, dt } = *frame_context;

        if events.rmb_down {
            let hdelta = -events.mouse_delta.0 as f32 * dt;
            self.camera.transform
                .rotate_around_axis(Vector3::unit_y(), Rad(hdelta), TransformSpace::World);

            let forward = self.camera.transform.forward();
            let angle_to_up = forward.angle(Vector3::unit_y()).0;
            let mut vdelta = -events.mouse_delta.1 as f32 * dt;
            if vdelta < 0.0 { // Moving up
                if angle_to_up + vdelta <= 0.1 {
                    vdelta = -(angle_to_up - 0.1);
                }
            } else if angle_to_up + vdelta >= 3.04 {
                vdelta = 3.04 - angle_to_up;
            }

            self.camera.transform
                .rotate_around_axis(Vector3::unit_x(), Rad(vdelta), TransformSpace::Local);
        }

        let mut movement: Vector3<f32> = Vector3::zero();
        if events.forward_down {
            movement -= Vector3::unit_z();
        }
        if events.back_down {
            movement += Vector3::unit_z();
        }
        if events.right_down {
            movement += Vector3::unit_x();
        }
        if events.left_down {
            movement -= Vector3::unit_x();
        }
        if events.up_down {
            movement += Vector3::unit_y();
        }
        if events.down_down {
            movement -= Vector3::unit_y();
        }
        if !movement.is_zero() {
            self.camera.transform
                .translate(movement.normalize() * dt * 10.0);
        }
    }
}


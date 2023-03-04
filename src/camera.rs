use cgmath::{InnerSpace, Matrix4, Rad, SquareMatrix, Vector3, Zero};
use crate::input::Input;
use crate::transform::{Transform, TransformSpace};

pub struct Camera {
    aspect: f32,
    fovy: f32,
    znear: f32,
    zfar: f32,
    transform: Transform,
}

impl Camera {
    pub fn new(eye: Vector3<f32>, target: Vector3<f32>, canvas_size: (f32, f32)) -> Self {
        let mut cam = Self {
            aspect: canvas_size.0 / canvas_size.1,
            fovy: 45.0,
            znear: 0.1,
            zfar: 100.0,
            transform: Transform::new(Vector3::zero()),
        };
        cam.transform.look_at(eye, target);
        cam
    }

    pub fn view_proj_matrix(&self) -> Matrix4<f32> {
        let proj = cgmath::perspective(cgmath::Deg(self.fovy), self.aspect, self.znear, self.zfar);
        proj * self.transform.matrix().invert().unwrap()
    }

    pub fn update(&mut self, input: &Input, dt: f32) {
        if input.rmb_down {
            let hdelta = input.mouse_delta.0 as f32 * dt;
            self.transform.rotate_around_axis(Vector3::unit_y(), -Rad(hdelta), TransformSpace::World);

            let forward = self.transform.forward();
            let angle_to_up = forward.angle(Vector3::unit_y()).0;
            let mut vdelta = input.mouse_delta.1 as f32 * dt;
            // TODO Fix, this does not work: when moving upward the angle usually doesn't even approach zero,
            // maybe the angle calculation is off.
            if vdelta < 0.0 { // Moving up
                if angle_to_up + vdelta <= 0.1 {
                    vdelta = -(angle_to_up - 0.1);
                }
            } else if angle_to_up + vdelta >= 3.04 {
                vdelta = 3.04 - angle_to_up;
            }
            self.transform.rotate_around_axis(Vector3::unit_x(), -Rad(vdelta), TransformSpace::Local);
        }

        let mut movement: Vector3<f32> = Vector3::zero();
        if input.forward_down {
            movement -= Vector3::unit_z();
        }
        if input.back_down {
            movement += Vector3::unit_z();
        }
        if input.right_down {
            movement += Vector3::unit_x();
        }
        if input.left_down {
            movement -= Vector3::unit_x();
        }
        if input.up_down {
            movement += Vector3::unit_y();
        }
        if input.down_down {
            movement -= Vector3::unit_y();
        }
        if !movement.is_zero() {
            self.transform.translate(movement.normalize() * dt * 10.0);
        }
    }
}

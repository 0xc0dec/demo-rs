use cgmath::{Array, Deg, InnerSpace, Matrix4, Rad, SquareMatrix, Vector3, Zero};
use crate::device::SurfaceSize;
use crate::events::Events;
use crate::transform::{Transform, TransformSpace};

pub struct Camera {
    aspect: f32,
    znear: f32,
    zfar: f32,
    fov: Deg<f32>,
    proj_matrix: Matrix4<f32>,
    transform: Transform,
}

impl Camera {
    pub fn new(pos: Vector3<f32>, target: Vector3<f32>, canvas_size: (f32, f32)) -> Self {
        let mut transform = Transform::new(pos, Vector3::from_value(1.0));
        transform.look_at(target);

        let aspect = canvas_size.0 / canvas_size.1;
        let znear = 0.1;
        let zfar = 100.0;
        let fov = cgmath::Deg(45.0);
        let proj_matrix = cgmath::perspective(fov, aspect, znear, zfar);

        Self {
            aspect,
            znear,
            zfar,
            fov,
            proj_matrix,
            transform,
        }
    }

    pub fn proj_matrix(&self) -> Matrix4<f32> { self.proj_matrix }
    pub fn view_matrix(&self) -> Matrix4<f32> { self.transform.matrix().invert().unwrap() }
    pub fn view_proj_matrix(&self) -> Matrix4<f32> { self.proj_matrix * self.view_matrix() }

    // TODO Rename to set_size() or set_fov() or smth because camera should not know about "surfaces".
    pub fn on_surface_resize(&mut self, new_size: SurfaceSize) {
        self.aspect = new_size.width as f32 / new_size.height as f32;
        self.proj_matrix = cgmath::perspective(self.fov, self.aspect, self.znear, self.zfar)
    }

    pub fn update(&mut self, events: &Events, dt: f32) {
        if events.rmb_down {
            let hdelta = -events.mouse_delta.0 as f32 * dt;
            self.transform.rotate_around_axis(Vector3::unit_y(), Rad(hdelta), TransformSpace::World);

            let forward = self.transform.forward();
            let angle_to_up = forward.angle(Vector3::unit_y()).0;
            let mut vdelta = -events.mouse_delta.1 as f32 * dt;
            if vdelta < 0.0 { // Moving up
                if angle_to_up + vdelta <= 0.1 {
                    vdelta = -(angle_to_up - 0.1);
                }
            } else if angle_to_up + vdelta >= 3.04 {
                vdelta = 3.04 - angle_to_up;
            }
            self.transform.rotate_around_axis(Vector3::unit_x(), Rad(vdelta), TransformSpace::Local);
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
            self.transform.translate(movement.normalize() * dt * 10.0);
        }
    }
}

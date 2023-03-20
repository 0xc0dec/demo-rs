use crate::transform::Transform;
use cgmath::*;
use rapier3d::na;
use crate::math::{Degrees, from_na_matrix, Mat4, Mat4_, Vec3};

pub struct Camera {
    aspect: f32,
    znear: f32,
    zfar: f32,
    fov: Degrees,
    proj_matrix: Mat4,
    pub transform: Transform,
}

impl Camera {
    pub fn new(pos: Vec3, target: Vec3, canvas_size: (f32, f32)) -> Self {
        let mut transform = Transform::new(pos, Vec3::from_element(1.0));
        transform.look_at(target);

        let aspect = canvas_size.0 / canvas_size.1;
        let znear = 0.1;
        let zfar = 100.0;
        let fov = Deg(45.0);
        let proj_matrix = from_na_matrix(
            na::Perspective3::new(aspect, fov.0, znear, zfar).to_homogeneous()
        );

        Self {
            aspect,
            znear,
            zfar,
            fov,
            proj_matrix,
            transform,
        }
    }

    pub fn proj_matrix(&self) -> Mat4 {
        self.proj_matrix
    }

    pub fn view_matrix(&self) -> Mat4_ {
        self.transform.matrix2().try_inverse().unwrap()
    }

    pub fn view_proj_matrix(&self) -> Mat4 {
        self.proj_matrix * from_na_matrix(self.view_matrix())
    }

    pub fn set_fov(&mut self, width: f32, height: f32) {
        self.aspect = width / height;
        self.proj_matrix = from_na_matrix(
            na::Perspective3::new(self.aspect, self.fov.0, self.znear, self.zfar).to_homogeneous()
        );
    }
}

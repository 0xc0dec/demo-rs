use bevy_ecs::prelude::Component;
use crate::components::transform::Transform;
use rapier3d::na;
use crate::math::{Mat4, Vec3};

#[derive(Component)]
pub struct Camera {
    aspect: f32,
    znear: f32,
    zfar: f32,
    fov: f32,
    proj_matrix: Mat4,
    // TODO Get rid of this and use as proper component
    pub transform: Transform,
}

impl Camera {
    pub fn new(pos: Vec3, target: Vec3, canvas_size: (f32, f32)) -> Self {
        let mut transform = Transform::from_pos(pos);
        transform.look_at(target);

        let aspect = canvas_size.0 / canvas_size.1;
        let znear = 0.1;
        let zfar = 100.0;
        let fov = 45.0;
        let proj_matrix = na::Perspective3::new(aspect, fov, znear, zfar).to_homogeneous();

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

    pub fn view_matrix(&self) -> Mat4 {
        self.transform.matrix().try_inverse().unwrap()
    }

    pub fn view_proj_matrix(&self) -> Mat4 {
        self.proj_matrix * self.view_matrix()
    }

    pub fn set_fov(&mut self, width: f32, height: f32) {
        self.aspect = width / height;
        self.proj_matrix = na::Perspective3::new(self.aspect, self.fov, self.znear, self.zfar)
            .to_homogeneous();
    }
}

use bevy_ecs::prelude::Component;
use rapier3d::na;
use crate::math::Mat4;

#[derive(Component)]
pub struct Camera {
    aspect: f32,
    znear: f32,
    zfar: f32,
    fov: f32,
    proj_matrix: Mat4,
    // Tags to render via this camera
    pub render_tags: u32,
}

impl Camera {
    pub fn new(canvas_size: (f32, f32)) -> Self {
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
            render_tags: !0 // render all
        }
    }

    pub fn proj_matrix(&self) -> Mat4 {
        self.proj_matrix
    }

    pub fn set_fov(&mut self, width: f32, height: f32) {
        self.aspect = width / height;
        self.proj_matrix = na::Perspective3::new(self.aspect, self.fov, self.znear, self.zfar)
            .to_homogeneous();
    }
}

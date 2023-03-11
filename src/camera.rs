use cgmath::*;
use crate::transform::Transform;

pub struct Camera {
    aspect: f32,
    znear: f32,
    zfar: f32,
    fov: Deg<f32>,
    proj_matrix: Matrix4<f32>,
    pub transform: Transform,
}

impl Camera {
    pub fn new(pos: Vector3<f32>, target: Vector3<f32>, canvas_size: (f32, f32)) -> Self {
        let mut transform = Transform::new(pos, Vector3::from_value(1.0));
        transform.look_at(target);

        let aspect = canvas_size.0 / canvas_size.1;
        let znear = 0.1;
        let zfar = 100.0;
        let fov = Deg(45.0);
        let proj_matrix = perspective(fov, aspect, znear, zfar);

        Self {
            aspect,
            znear,
            zfar,
            fov,
            proj_matrix,
            transform,
        }
    }

    pub fn proj_matrix(&self) -> Matrix4<f32> {
        self.proj_matrix
    }

    pub fn view_matrix(&self) -> Matrix4<f32> {
        self.transform.matrix().invert().unwrap()
    }

    pub fn view_proj_matrix(&self) -> Matrix4<f32> {
        self.proj_matrix * self.view_matrix()
    }

    pub fn set_fov(&mut self, width: f32, height: f32) {
        self.aspect = width / height;
        self.proj_matrix = perspective(self.fov, self.aspect, self.znear, self.zfar)
    }
}

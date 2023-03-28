use rapier3d::na;

pub type Vec3 = na::Vector3<f32>;
pub type Mat4 = na::Matrix4<f32>;
pub type Quat = na::Quaternion<f32>;
pub type UnitQuat = na::UnitQuaternion<f32>;

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: Mat4 = Mat4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.5,
    0.0, 0.0, 0.0, 1.0,
);
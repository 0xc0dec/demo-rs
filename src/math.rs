use std::ops::Add;

use rapier3d::na;
use rapier3d::na::Point3;

pub type Vec2 = na::Vector2<f32>;
pub type Vec3 = na::Vector3<f32>;
pub type Mat4 = na::Matrix4<f32>;
pub type Quat = na::Quaternion<f32>;
pub type UnitQuat = na::UnitQuaternion<f32>;
pub use rapier3d::prelude::Ray;

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: Mat4 = Mat4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.5,
    0.0, 0.0, 0.0, 1.0,
);

// TODO Is there a better way to cast?
pub fn to_point3(v: Vec3) -> Point3<f32> {
    Point3::origin().add(v)
}

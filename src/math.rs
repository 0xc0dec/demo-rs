use cgmath::{Deg, Matrix4, Quaternion, Vector3};
use rapier3d::math::Rotation;
use rapier3d::na::Point3;
use rapier3d::prelude::{Point, Real, Vector};

pub fn to_na_point(vec: Vec3) -> Point<Real> {
    Point::new(vec.x, vec.y, vec.z)
}

pub fn from_na_point(pt: Point3<Real>) -> Vec3 {
    Vec3::new(pt.x, pt.y, pt.z)
}

pub fn to_na_vec3(vec: Vec3) -> Vector<Real> {
    Vector::new(vec.x, vec.y, vec.z)
}

pub fn from_na_vec3(vec: Vector<Real>) -> Vec3 {
    Vec3::new(vec.x, vec.y, vec.z)
}

pub fn from_na_rot(r: Rotation<Real>) -> Quat {
    Quat::new(r.i, r.j, r.k, r.w)
}

pub type Vec3 = Vector3<f32>;
pub type Mat4 = Matrix4<f32>;
pub type Quat = Quaternion<f32>;
pub type Degrees = Deg<f32>;
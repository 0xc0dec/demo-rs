use cgmath::Vector3;
use rapier3d::na::Point3;
use rapier3d::prelude::{Point, Real, Vector};

pub fn to_na_vec3(vec: Vector3<f32>) -> Vector<Real> {
    Vector::new(vec.x, vec.y, vec.z)
}

pub fn to_na_point(vec: Vector3<f32>) -> Point<Real> {
    Point::new(vec.x, vec.y, vec.z)
}

pub fn from_na_vec3(vec: Vector<Real>) -> Vector3<f32> {
    Vector3::new(vec.x, vec.y, vec.z)
}

pub fn from_na_point(pt: Point3<Real>) -> Vector3<f32> {
    Vector3::new(pt.x, pt.y, pt.z)
}
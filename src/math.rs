use cgmath::Vector3;
use rapier3d::prelude::{Real, Vector};

pub fn to_na_vec3(vec: Vector3<f32>) -> Vector<Real> {
    Vector::new(vec.x, vec.y, vec.z)
}
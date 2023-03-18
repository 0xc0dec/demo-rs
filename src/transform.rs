use cgmath::{ElementWise, InnerSpace, Matrix, Rad, Rotation, Transform as _, Vector3};
use crate::math::{Mat4, Quat, Vec3};

pub enum TransformSpace {
    Local,
    World,
}

pub struct Transform {
    m: Mat4,
    scale: Vec3,
}

// TODO Parent-child relationships
impl Transform {
    pub fn new(pos: Vec3, scale: Vec3) -> Self {
        let m = Mat4::from_translation(pos)
            * Mat4::from_nonuniform_scale(scale.x, scale.y, scale.z);
        Self { m, scale }
    }

    pub fn matrix(&self) -> Mat4 {
        self.m
    }

    pub fn forward(&self) -> Vec3 {
        self.m.z.truncate()
    }

    pub fn right(&self) -> Vec3 {
        self.m.x.truncate()
    }

    pub fn up(&self) -> Vec3 {
        self.m.y.truncate()
    }

    pub fn position(&self) -> Vec3 {
        self.m.w.truncate()
    }

    pub fn look_at(&mut self, target: Vec3) {
        // For some reason could not make it work with Mat4::look_at, was getting weird results.
        let rot_mtx = Mat4::from(Quat::look_at(
            self.position() - target,
            Vec3::unit_y(),
        ))
            .transpose()
            * Mat4::from_nonuniform_scale(self.scale.x, self.scale.y, self.scale.z);
        self.m.x = rot_mtx.x;
        self.m.y = rot_mtx.y;
        self.m.z = rot_mtx.z;
    }

    pub fn translate(&mut self, v: Vec3) {
        self.m.w.x += v.x;
        self.m.w.y += v.y;
        self.m.w.z += v.z;
    }

    pub fn set_position(&mut self, pos: Vec3) {
        self.m.w.x = pos.x;
        self.m.w.y = pos.y;
        self.m.w.z = pos.z;
    }

    pub fn set_scale(&mut self, scale: Vec3) {
        self.m.x = self.m.x.normalize().mul_element_wise(scale.extend(1.0));
        self.m.y = self.m.y.normalize().mul_element_wise(scale.extend(1.0));
        self.m.z = self.m.z.normalize().mul_element_wise(scale.extend(1.0));
    }

    pub fn set(&mut self, pos: Vec3, rotation: Quat) {
        self.m = Mat4::from_translation(pos);

        // Again, not sure why transposition is needed
        let rot_mtx = Mat4::from(rotation).transpose()
            * Mat4::from_nonuniform_scale(self.scale.x, self.scale.y, self.scale.z);
        self.m.x = rot_mtx.x;
        self.m.y = rot_mtx.y;
        self.m.z = rot_mtx.z;
    }

    pub fn rotate_around_axis(
        &mut self,
        axis: Vec3,
        angle: Rad<f32>,
        space: TransformSpace,
    ) {
        let axis = axis.normalize();
        self.m = self.m * match space {
            TransformSpace::Local => Mat4::from_axis_angle(axis, angle),
            TransformSpace::World => {
                let axis = self.m.inverse_transform_vector(axis).unwrap();
                Mat4::from_axis_angle(axis, angle)
            }
        };
    }
}

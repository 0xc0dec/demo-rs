use cgmath::{InnerSpace, Rad, Transform as _};
use rapier3d::na;
use crate::math::{from_na_matrix, from_na_vec3, Mat4, Mat4_, Quat, to_na_matrix, to_na_vec3, Vec3};

pub enum TransformSpace {
    Local,
    World,
}

pub struct Transform {
    m: Mat4,
    m2: Mat4_,
    scale: Vec3,
    pos: Vec3,
    rot: na::UnitQuaternion<f32>
}

// TODO Parent-child relationships
impl Transform {
    pub fn new(pos: Vec3, scale: Vec3) -> Self {
        let m = na::Matrix4::identity();
        let rot = na::UnitQuaternion::identity();
        let mut res = Self {
            m: from_na_matrix(m),
            m2: m,
            rot,
            scale,
            pos
        };
        res.rebuild_matrix();
        res
    }

    pub fn matrix(&self) -> Mat4 {
        self.m
    }

    pub fn matrix2(&self) -> Mat4_ {
        self.m2
    }

    pub fn forward(&self) -> Vec3 {
        let m = to_na_matrix(&self.m);
        -from_na_vec3(m.column(2).xyz())
    }

    pub fn right(&self) -> Vec3 {
        let m = to_na_matrix(&self.m);
        from_na_vec3(m.column(0).xyz())
    }

    pub fn up(&self) -> Vec3 {
        let m = to_na_matrix(&self.m);
        from_na_vec3(m.column(1).xyz())
    }

    pub fn position(&self) -> Vec3 {
        let m = to_na_matrix(&self.m);
        from_na_vec3(m.column(3).xyz())
    }

    pub fn look_at(&mut self, target: Vec3) {
        self.rot = na::UnitQuaternion::look_at_rh(
            &to_na_vec3(target - self.pos),
            &na::Vector3::new(0.0, 1.0, 0.0)
        );
        self.rebuild_matrix();
    }

    pub fn translate(&mut self, v: Vec3) {
        let mut m = to_na_matrix(&self.m);
        m.append_translation_mut(&to_na_vec3(v));
        self.m2 = m;
        self.m = from_na_matrix(self.m2);
        self.pos += v;
    }

    pub fn set_position(&mut self, pos: Vec3) {
        self.pos = pos;
        self.rebuild_matrix();
    }

    pub fn set_scale(&mut self, scale: Vec3) {
        self.scale = scale;
        self.rebuild_matrix();
    }

    // TODO Fix, the visuals don't always match physics
    pub fn set(&mut self, pos: Vec3, rotation: Quat) {
        self.rot = na::UnitQuaternion::from_quaternion(
            na::Quaternion::new(rotation.v.x, rotation.v.y, rotation.v.z, rotation.s)
        );
        self.pos = pos;
        self.rebuild_matrix();
    }

    pub fn rotate_around_axis(
        &mut self,
        axis: Vec3,
        angle: Rad<f32>,
        space: TransformSpace,
    ) {
        let axis = axis.normalize();
        let axis = match space {
            TransformSpace::Local => axis,
            TransformSpace::World => self.m.inverse_transform_vector(axis).unwrap()
        };

        self.rot = na::UnitQuaternion::from_scaled_axis(to_na_vec3(axis * angle.0)) * self.rot;
        self.rebuild_matrix();
    }

    fn rebuild_matrix(&mut self) {
        let rot_m = na::Rotation3::from(self.rot).transpose();
        let tr_m = na::Translation3::new(self.pos.x, self.pos.y, self.pos.z);
        let rot_and_tr_m = tr_m * rot_m;
        self.m2 = rot_and_tr_m
            .to_matrix()
            .prepend_nonuniform_scaling(&to_na_vec3(self.scale));
        self.m = from_na_matrix(self.m2);
    }
}

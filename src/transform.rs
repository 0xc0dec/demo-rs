use cgmath::{InnerSpace, Matrix, Matrix4, Quaternion, Rad, Rotation, Transform as _, Vector3};

pub enum TransformSpace {
    Local,
    World,
}

pub struct Transform {
    m: Matrix4<f32>,
    scale: Vector3<f32>,
}

// TODO Parent-child relationships
impl Transform {
    pub fn new(pos: Vector3<f32>, scale: Vector3<f32>) -> Self {
        let m = Matrix4::from_translation(pos)
            * Matrix4::from_nonuniform_scale(scale.x, scale.y, scale.z);
        Self { m, scale }
    }

    pub fn matrix(&self) -> Matrix4<f32> {
        self.m
    }

    pub fn forward(&self) -> Vector3<f32> {
        self.m.z.truncate()
    }

    pub fn right(&self) -> Vector3<f32> {
        self.m.x.truncate()
    }

    pub fn up(&self) -> Vector3<f32> {
        self.m.y.truncate()
    }

    pub fn position(&self) -> Vector3<f32> {
        self.m.w.truncate()
    }

    pub fn look_at(&mut self, target: Vector3<f32>) {
        // For some reason could not make it work with Matrix4::look_at, was getting weird results.
        let rot_mtx = Matrix4::from(Quaternion::look_at(
            self.position() - target,
            Vector3::unit_y(),
        ))
            .transpose()
            * Matrix4::from_nonuniform_scale(self.scale.x, self.scale.y, self.scale.z);
        self.m.x = rot_mtx.x;
        self.m.y = rot_mtx.y;
        self.m.z = rot_mtx.z;
    }

    pub fn translate(&mut self, v: Vector3<f32>) {
        self.m.w.x += v.x;
        self.m.w.y += v.y;
        self.m.w.z += v.z;
    }

    pub fn set_position(&mut self, pos: Vector3<f32>) {
        self.m.w.x = pos.x;
        self.m.w.y = pos.y;
        self.m.w.z = pos.z;
    }

    pub fn set(&mut self, pos: Vector3<f32>, rotation: Quaternion<f32>) {
        self.m = Matrix4::from_translation(pos);

        // Again, not sure why transposition is needed
        let rot_mtx = Matrix4::from(rotation).transpose()
            * Matrix4::from_nonuniform_scale(self.scale.x, self.scale.y, self.scale.z);
        self.m.x = rot_mtx.x;
        self.m.y = rot_mtx.y;
        self.m.z = rot_mtx.z;
    }

    pub fn rotate_around_axis(
        &mut self,
        axis: Vector3<f32>,
        angle: Rad<f32>,
        space: TransformSpace,
    ) {
        let axis = axis.normalize();
        self.m = self.m
            * match space {
            TransformSpace::Local => Matrix4::from_axis_angle(axis, angle),
            TransformSpace::World => {
                let axis = self.m.inverse_transform_vector(axis).unwrap();
                Matrix4::from_axis_angle(axis, angle)
            }
        };
    }
}

use cgmath::{InnerSpace, Matrix, Matrix4, Quaternion, Rad, Rotation, Transform as _, Vector3};

pub enum TransformSpace {
    Local,
    World
}

pub struct Transform {
    m: Matrix4<f32>,
    pos: Vector3<f32>,
}

// TODO Parent-child relationships
impl Transform {
    pub fn new(pos: Vector3<f32>) -> Self {
        Self {
            m: Matrix4::from_translation(pos),
            pos
        }
    }

    pub fn matrix(&self) -> Matrix4<f32> { self.m }
    pub fn forward(&self) -> Vector3<f32> { self.m.z.truncate() }

    pub fn look_at(&mut self, target: Vector3<f32>) {
        // For some reason could not make it work with Matrix4::look_at, was getting weird results.
        let rot_mtx = Matrix4::from(Quaternion::look_at(self.pos - target, Vector3::unit_y())).transpose();
        self.m.x = rot_mtx.x;
        self.m.y = rot_mtx.y;
        self.m.z = rot_mtx.z;
    }

    // TODO Specify space
    pub fn translate(&mut self, v: Vector3<f32>) {
        self.m = self.m * Matrix4::from_translation(v);
        self.pos += v;
    }

    pub fn set(&mut self, pos: Vector3<f32>, rotation: Quaternion<f32>) {
        self.pos = pos;
        self.m = Matrix4::from_translation(self.pos);

        let rot_mtx = Matrix4::from(rotation);
        self.m.x = rot_mtx.x;
        self.m.y = rot_mtx.y;
        self.m.z = rot_mtx.z;
    }

    pub fn rotate_around_axis(&mut self, axis: Vector3<f32>, angle: Rad<f32>, space: TransformSpace) {
        let axis = axis.normalize();
        self.m = self.m * match space {
            TransformSpace::Local => Matrix4::from_axis_angle(axis, angle),
            TransformSpace::World => {
                let axis = self.m.inverse_transform_vector(axis).unwrap();
                Matrix4::from_axis_angle(axis, angle)
            },
        };
    }
}

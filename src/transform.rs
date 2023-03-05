use cgmath::{Array, EuclideanSpace, InnerSpace, Matrix4, Point3, Rad, Transform as _, Vector3};

pub enum TransformSpace {
    Local,
    World
}

pub struct Transform {
    m: Matrix4<f32>,
}

// TODO Parent-child relationships
impl Transform {
    pub fn new(pos: Vector3<f32>) -> Self {
        Self {
            m: Matrix4::from_translation(pos)
        }
    }

    pub fn matrix(&self) -> Matrix4<f32> { self.m }
    pub fn position(&self) -> Vector3<f32> { self.m.transform_point(Point3::from_value(0.0)).to_vec() }

    pub fn look_at(&mut self, pos: Vector3<f32>, target: Vector3<f32>) {
        self.m = Matrix4::look_at_rh(
            Point3::from_vec(pos),
            Point3::from_vec(target),
            Vector3::unit_y()
        );
    }

    pub fn translate(&mut self, v: Vector3<f32>) {
        self.m = self.m * Matrix4::from_translation(v);
    }

    pub fn forward(&self) -> Vector3<f32> {
        self.m.z.truncate()
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

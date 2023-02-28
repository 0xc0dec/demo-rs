use cgmath::{EuclideanSpace, Matrix4, Point3, Quaternion, Rad, Rotation3, Transform as _, Vector3};

pub enum TransformSpace {
    Local,
    World
}

pub struct Transform {
    local_mat: Matrix4<f32>,
}

// TODO Parent-child relationships
impl Transform {
    pub fn new() -> Self {
        Self {
            local_mat: Matrix4::look_at_rh(
                Point3::new(0.0, 0.0, 0.0),
                Point3::new(0.0, 0.0, 1.0),
                Vector3::unit_y()
            ),
        }
    }

    pub fn matrix(&self) -> Matrix4<f32> {
        self.local_mat
    }

    pub fn look_at(&mut self, pos: Vector3<f32>, target: Vector3<f32>) {
        self.local_mat = Matrix4::look_at_rh(
            Point3::from_vec(pos),
            Point3::from_vec(target),
            Vector3::unit_y()
        );
    }

    pub fn translate(&mut self, v: Vector3<f32>) {
        let t = Matrix4::from_translation(v);
        self.local_mat = t * self.local_mat;
    }

    pub fn forward(&self) -> Vector3<f32> {
        self.local_mat.z.truncate()
    }

    pub fn rotate_around_axis(&mut self, axis: Vector3<f32>, angle: Rad<f32>, space: TransformSpace) {
        let axis = match space {
            TransformSpace::Local => axis,
            TransformSpace::World => self.local_mat.transform_vector(axis), // TODO why not inverse?
        };
        self.local_mat = Matrix4::from_axis_angle(axis, angle) * self.local_mat;
    }
}

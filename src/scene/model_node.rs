use cgmath::{Deg, Quaternion, Rad, Vector3, Zero};
use rapier3d::prelude::*;
use wgpu::RenderPass;
use crate::camera::Camera;
use crate::driver::Driver;
use crate::materials::{DiffuseMaterial, DiffuseMaterialParams, Material};
use crate::model::{DrawModel, Model};
use crate::physics::PhysicsWorld;
use crate::texture::Texture;
use super::scene_node::SceneNode;
use crate::transform::{Transform, TransformSpace};

pub struct ModelNode {
    model: Model,
    transform: Transform,
    material: DiffuseMaterial,
    rigid_body_handle: RigidBodyHandle,
}

impl ModelNode {
    pub async fn new(pos: Vector3<f32>, driver: &Driver, physics: &mut PhysicsWorld) -> Self {
        let mut body = RigidBodyBuilder::dynamic()
            .translation(vector![pos.x, pos.y, pos.z])
            .build();
        body.add_torque(Vector::new(10.0, 10.0, 10.0), true);

        let collider = ColliderBuilder::cuboid(1.0, 1.0, 1.0).restitution(0.7).build();
        let rigid_body_handle = physics.add_body(body, collider);

        let model = Model::from_file("cube.obj", driver).await.expect("Failed to load cube model");
        let transform = Transform::new(pos);

        let texture = Texture::from_file_2d("stonewall.jpg", driver).await.unwrap();
        let material = DiffuseMaterial::new(driver, DiffuseMaterialParams { texture }).await;

        Self {
            model,
            transform,
            material,
            rigid_body_handle
        }
    }
}

impl SceneNode for ModelNode {
    fn update(&mut self, dt: f32, physics: &PhysicsWorld) {
        let body = physics.rigid_body_set().get(self.rigid_body_handle).unwrap();
        let phys_pos = body.translation();
        let phys_rot = body.rotation();

        self.transform.set(
            Vector3::new(phys_pos.x, phys_pos.y, phys_pos.z),
            Quaternion::new(phys_rot.i, phys_rot.j, phys_rot.k, phys_rot.w)
        );

        // self.transform.rotate_around_axis(
        //     Vector3::unit_z(),
        //     Rad::from(Deg(45.0 * dt)),
        //     TransformSpace::Local)
    }

    fn render<'a, 'b>(&'a mut self, driver: &'a Driver, camera: &'a Camera, pass: &mut RenderPass<'b>)
        where 'a: 'b
    {
        self.material.update(driver, camera, &self.transform);
        self.material.apply(pass);
        pass.draw_model(&self.model);
    }
}
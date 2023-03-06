use cgmath::{Deg, Rad, Vector3, Zero};
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
}

impl ModelNode {
    pub async fn new(pos: Vector3<f32>, driver: &Driver, physics: &mut PhysicsWorld) -> Self {
        Self {
            model: Model::from_file("cube.obj", driver).await.expect("Failed to load cube model"),
            transform: Transform::new(pos),
            material: {
                let texture = Texture::from_file_2d("stonewall.jpg", driver).await.unwrap();
                DiffuseMaterial::new(driver, DiffuseMaterialParams { texture }).await
            },
        }
    }
}

impl SceneNode for ModelNode {
    fn update(&mut self, dt: f32) {
        self.transform.rotate_around_axis(
            Vector3::unit_z(),
            Rad::from(Deg(45.0 * dt)),
            TransformSpace::Local)
    }

    fn render<'a, 'b>(&'a mut self, driver: &'a Driver, camera: &'a Camera, pass: &mut RenderPass<'b>)
        where 'a: 'b
    {
        self.material.update(driver, camera, &self.transform);
        self.material.apply(pass);
        pass.draw_model(&self.model);
    }
}
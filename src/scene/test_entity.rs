use cgmath::{Quaternion, Vector3};
use rapier3d::prelude::*;
use crate::camera::Camera;
use crate::graphics::Graphics;
use crate::materials::{DiffuseMaterial, DiffuseMaterialParams, Material};
use crate::model::{DrawModel, Model};
use crate::physics::PhysicsWorld;
use crate::texture::Texture;
use super::entity::Entity;
use crate::transform::{Transform};

pub struct TestEntity {
    model: Model,
    transform: Transform,
    material: DiffuseMaterial,
    rigid_body_handle: RigidBodyHandle,
}

pub struct TestEntityParams {
    pub pos: Vector3<f32>,
    pub scale: Vector3<f32>,
    pub movable: bool,
}

impl TestEntity {
    pub async fn new(gfx: &Graphics, physics: &mut PhysicsWorld, params: TestEntityParams) -> Self {
        let body = if params.movable { RigidBodyBuilder::dynamic() } else { RigidBodyBuilder::fixed() }
            .translation(vector![params.pos.x, params.pos.y, params.pos.z])
            .build();
        let collider = ColliderBuilder::cuboid(params.scale.x, params.scale.y, params.scale.z)
            .restitution(0.7)
            .build();
        let rigid_body_handle = physics.add_body(body, collider);

        let transform = Transform::new(params.pos, params.scale);

        let model = Model::from_file("cube.obj", gfx).await.expect("Failed to load cube model");
        let texture = Texture::from_file_2d("stonewall.jpg", gfx).await.unwrap();
        let material = DiffuseMaterial::new(gfx, DiffuseMaterialParams { texture }).await;

        Self {
            model,
            transform,
            material,
            rigid_body_handle
        }
    }
}

impl Entity for TestEntity {
    fn update(&mut self, _dt: f32, physics: &PhysicsWorld) {
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

    fn render<'a, 'b>(&'a mut self, gfx: &'a Graphics, camera: &'a Camera, pass: &mut wgpu::RenderPass<'b>)
        where 'a: 'b
    {
        self.material.update(gfx, camera, &self.transform);
        self.material.apply(pass);
        pass.draw_model(&self.model);
    }
}
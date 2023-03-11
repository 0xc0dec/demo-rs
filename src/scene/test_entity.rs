use cgmath::{Quaternion, Vector3};
use rapier3d::prelude::*;
use crate::camera::Camera;
use crate::device::Device;
use crate::model::{DrawModel, Model};
use crate::physics::PhysicsWorld;
use crate::shaders::{DiffuseShader, DiffuseShaderParams, Shader};
use crate::texture::Texture;
use super::entity::Entity;
use crate::transform::{Transform};

pub struct TestEntity {
    model: Model,
    transform: Transform,
    shader: DiffuseShader,
    rigid_body_handle: RigidBodyHandle,
}

pub struct TestEntityParams {
    pub pos: Vector3<f32>,
    pub scale: Vector3<f32>,
    pub movable: bool,
}

impl TestEntity {
    pub async fn new(gfx: &Device, physics: &mut PhysicsWorld, params: TestEntityParams) -> Self {
        let body = if params.movable { RigidBodyBuilder::dynamic() } else { RigidBodyBuilder::fixed() }
            .translation(vector![params.pos.x, params.pos.y, params.pos.z])
            .build();
        let collider = ColliderBuilder::cuboid(params.scale.x, params.scale.y, params.scale.z)
            .restitution(0.7)
            .build();
        let rigid_body_handle = physics.add_body(body, collider);

        let transform = Transform::new(params.pos, params.scale);

        let model = Model::from_file("cube.obj", gfx).await.expect("Failed to load cube model");
        let texture = Texture::new_2d_from_file("stonewall.jpg", gfx).await.unwrap();
        let shader = DiffuseShader::new(gfx, DiffuseShaderParams { texture }).await;

        Self {
            model,
            transform,
            shader,
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

    fn render<'a, 'b>(&'a mut self, gfx: &'a Device, camera: &'a Camera, encoder: &mut wgpu::RenderBundleEncoder<'b>)
        where 'a: 'b
    {
        self.shader.update(gfx, camera, &self.transform);
        self.shader.apply(encoder);
        encoder.draw_model(&self.model);
    }
}
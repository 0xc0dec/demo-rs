use cgmath::{Quaternion, Vector3};
use rapier3d::prelude::*;
use crate::camera::Camera;
use crate::device::{Device, Frame};
use crate::model::{DrawModel, Model};
use crate::physics_world::PhysicsWorld;
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
    pub async fn new(device: &Device, physics: &mut PhysicsWorld, params: TestEntityParams) -> Self {
        let body = if params.movable { RigidBodyBuilder::dynamic() } else { RigidBodyBuilder::fixed() }
            .translation(vector![params.pos.x, params.pos.y, params.pos.z])
            .build();
        let collider = ColliderBuilder::cuboid(params.scale.x, params.scale.y, params.scale.z)
            .restitution(0.7)
            .friction(0.7)
            .build();
        let (rigid_body_handle, _) = physics.add_body(body, collider);

        let transform = Transform::new(params.pos, params.scale);

        let model = Model::from_file("cube.obj", device).await.expect("Failed to load cube model");
        let texture = Texture::new_2d_from_file("stonewall.jpg", device).await.unwrap();
        let shader = DiffuseShader::new(device, DiffuseShaderParams { texture }).await;

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
        let body = physics.bodies.get(self.rigid_body_handle).unwrap();
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

    fn render<'a, 'b>(&'a mut self, device: &'a Device, camera: &'a Camera, frame: &mut Frame<'b, 'a>)
        where 'a: 'b
    {
        self.shader.update(device, camera, &self.transform);
        self.shader.apply(frame);
        frame.draw_model(&self.model);
    }
}
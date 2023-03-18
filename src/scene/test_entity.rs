use std::rc::Rc;
use crate::camera::Camera;
use crate::device::{Device, Frame};
use crate::model::{DrawModel, Model};
use crate::physics_world::PhysicsWorld;
use crate::shaders::{DiffuseShader, DiffuseShaderParams, Shader};
use crate::transform::{Transform};
use cgmath::{Deg, Vector3};
use rapier3d::prelude::*;
use crate::math::{from_na_rot, from_na_vec3, to_na_vec3};
use crate::state::State;

pub struct TestEntity {
    model: Rc<Model>,
    transform: Transform,
    shader: DiffuseShader,
    rigid_body_handle: RigidBodyHandle,
}

pub struct TestEntityParams {
    pub pos: Vector3<f32>,
    pub scale: Vector3<f32>,
    pub rotation_angle: Deg<f32>,
    pub rotation_axis: Vector3<f32>,
    pub movable: bool,
}

impl TestEntity {
    pub async fn new(
        state: &mut State,
        physics: &mut PhysicsWorld,
        params: TestEntityParams,
    ) -> Self {
        let TestEntityParams {
            pos,
            scale,
            rotation_axis,
            rotation_angle,
            movable,
        } = params;

        let body = if movable { RigidBodyBuilder::dynamic() } else { RigidBodyBuilder::fixed() }
            .translation(vector![pos.x, pos.y, pos.z])
            // TODO Verify this conversion
            .rotation(to_na_vec3(rotation_axis) * rotation_angle.0)
            .build();
        let collider = ColliderBuilder::cuboid(scale.x, scale.y, scale.z)
            .restitution(0.2)
            .friction(0.7)
            .build();
        let (rigid_body_handle, _) = physics.add_body(body, collider);

        // Not rotating the transform because it'll get synced with the rigid body anyway
        let transform = Transform::new(pos, scale);
        let model = state.resources.model("cube.obj", &state.device).await;
        let texture = state.resources.texture_2d("stonewall.jpg", &state.device).await;
        let shader = DiffuseShader::new(
            &state.device,
            DiffuseShaderParams {
                texture: texture.as_ref()
            }
        ).await;

        Self {
            model,
            transform,
            shader,
            rigid_body_handle,
        }
    }

    pub fn update(&mut self, _dt: f32, physics: &PhysicsWorld) {
        let body = physics.bodies.get(self.rigid_body_handle).unwrap();
        let phys_pos = body.translation();
        let phys_rot = body.rotation();
        self.transform.set(from_na_vec3(*phys_pos), from_na_rot(*phys_rot));
    }

    pub fn render<'a, 'b>(
        &'a mut self,
        device: &'a Device,
        camera: &'a Camera,
        frame: &mut Frame<'b, 'a>,
    ) where
        'a: 'b,
    {
        self.shader.update(device, camera, &self.transform);
        self.shader.apply(frame);
        frame.draw_model(&self.model);
    }
}

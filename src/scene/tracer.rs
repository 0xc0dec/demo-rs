use crate::camera::Camera;
use crate::device::{Device, Frame};
use crate::model::{DrawModel, Model};
use crate::physics_world::PhysicsWorld;
use crate::scene::character::Character;
use crate::shaders::{ColorShader, Shader};
use crate::transform::Transform;
use cgmath::{Array, Vector3, Zero};

pub struct Tracer {
    model: Model,
    shader: ColorShader,
    transform: Transform,
    target_visible: bool,
}

impl Tracer {
    pub async fn new(device: &Device) -> Self {
        let model = Model::from_file("cube.obj", device)
            .await
            .expect("Failed to load cube model");
        let shader = ColorShader::new(device).await;
        let transform = Transform::new(Vector3::zero(), Vector3::from_value(0.2));

        Tracer {
            model,
            shader,
            transform,
            target_visible: false,
        }
    }

    pub fn update(&mut self, physics: &PhysicsWorld, character: &Character) {
        if let Some((_, hit_pt)) = physics.cast_ray(
            character.camera.transform.position(),
            // For some reason the ray needs to be inverted here, perhaps the physics engine uses
            // a different axis orientation?
            // TODO Somehow fix
            -character.camera.transform.forward(),
            Some(character.collider_handle),
        ) {
            self.target_visible = true;
            self.transform.set_position(hit_pt);
        } else {
            self.target_visible = false;
        }
    }

    pub fn render<'a, 'b>(
        &'a mut self,
        device: &'a Device,
        camera: &'a Camera,
        frame: &mut Frame<'b, 'a>,
    ) where
        'a: 'b,
    {
        if !self.target_visible {
            return;
        }

        self.shader.update(device, camera, &self.transform);
        self.shader.apply(frame);
        frame.draw_model(&self.model);
    }
}

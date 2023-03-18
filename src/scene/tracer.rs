use std::rc::Rc;
use crate::camera::Camera;
use crate::device::{Device, Frame};
use crate::model::{DrawModel, Model};
use crate::physics_world::PhysicsWorld;
use crate::scene::character::Character;
use crate::shaders::{ColorShader, Shader};
use crate::transform::Transform;
use cgmath::{Array, InnerSpace, Vector3, Zero};
use crate::app::App;
use crate::math::Vec3;

pub struct Tracer {
    model: Rc<Model>,
    shader: ColorShader,
    transform: Transform,
    target_visible: bool,
}

impl Tracer {
    pub async fn new(app: &mut App) -> Self {
        let model = app.resources.model("cube.obj", &app.device).await;
        let shader = ColorShader::new(&app.device).await;
        let transform = Transform::new(Vec3::zero(), Vec3::from_value(1.0));

        Tracer {
            model,
            shader,
            transform,
            target_visible: false,
        }
    }

    pub fn update(&mut self, physics: &PhysicsWorld, character: &Character) {
        if let Some((hit_pt, _, _)) = physics.cast_ray(
            character.camera.transform.position(),
            character.camera.transform.forward(),
            Some(character.collider_handle),
        ) {
            self.target_visible = true;
            self.transform.set_position(hit_pt);

            let dist_to_camera = (character.camera.transform.position() - hit_pt).magnitude();
            let scale = (dist_to_camera / 10.0).min(0.1).max(0.01);
            self.transform.set_scale(Vec3::from_value(scale));
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

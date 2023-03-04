use std::rc::Rc;
use cgmath::{Deg, Rad, Vector3};
use wgpu::RenderPass;
use crate::camera::Camera;
use crate::input::Input;
use crate::material::{Material, MaterialParams, RenderMaterial};
use crate::model::{DrawModel, Model};
use crate::driver::Driver;
use crate::texture::Texture;
use crate::transform::{Transform, TransformSpace};

struct SceneNode {
    model: Model,
    transform: Transform,
    material: Material
}

pub struct Scene {
    camera: Camera,
    nodes: Vec<SceneNode>
}

impl Scene {
    pub async fn new(device: &Driver) -> Scene {
        let camera = Camera::new(
            Vector3::new(5.0, 5.0, 5.0),
            Vector3::new(0.0, 0.0, 0.0),
            device.surface_size().into()
        );

        Self {
            camera,
            nodes: vec![
                SceneNode {
                    model: Model::from_file("cube.obj", device).await.expect("Failed to load cube model"),
                    transform: Transform::new(Vector3::unit_x() * 5.0),
                    material: {
                        let texture = Texture::from_file("cube-diffuse.jpg", device).await.unwrap();
                        Material::diffuse(device, MaterialParams { texture }).await
                    }
                },
                // TODO Avoid duplicate loading
                SceneNode {
                    model: Model::from_file("cube.obj", device).await.expect("Failed to load cube model"),
                    transform: Transform::new(Vector3::unit_z() * 5.0),
                    material: {
                        let texture = Texture::from_file("cube-diffuse.jpg", device).await.unwrap();
                        Material::diffuse(device, MaterialParams { texture }).await
                    }
                },
            ]
        }
    }

    pub fn update(&mut self, input: &Input, dt: f32) {
        self.camera.update(input, dt);
        for n in &mut self.nodes {
            n.transform.rotate_around_axis(
                Vector3::unit_z(),
                Rad::from(Deg(45.0 * dt)),
                TransformSpace::Local)
        }
    }

    pub fn render<'a, 'b>(&'a mut self, driver: &'a Driver, pass: &mut RenderPass<'b>)
        where 'a: 'b
    {
        for n in &mut self.nodes {
            n.material.update(driver, &self.camera, &n.transform);
            pass.apply_material(&n.material);
            pass.draw_model(&n.model);
        }
    }
}

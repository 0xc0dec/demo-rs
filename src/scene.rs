use cgmath::{Vector3};
use wgpu::RenderPass;
use crate::camera::Camera;
use crate::input::Input;
use crate::material::{Material, MaterialParams, RenderMaterial};
use crate::model::{self, DrawModel, Model};
use crate::renderer::Renderer;
use crate::texture::Texture;

pub struct Scene {
    pub material: Material,
    pub camera: Camera,
    model: model::Model,
}

impl Scene {
    pub async fn new(renderer: &Renderer) -> Scene {
        let texture = Texture::from_file("cube-diffuse.jpg", renderer).await.unwrap();
        let material = Material::diffuse(renderer, MaterialParams { texture }).await;

        let model = Model::from_file("cube.obj", renderer).await.expect("Failed to load cube model");

        let camera = Camera::new(
            Vector3::new(5.0, 5.0, 5.0),
            Vector3::new(0.0, 0.0, 0.0),
            renderer.canvas_size().into()
        );

        Self {
            material,
            camera,
            model,
        }
    }

    pub fn update(&mut self, input: &Input, dt: f32) {
        self.camera.update(input, dt);
    }

    pub fn render<'a, 'b>(&'a mut self, renderer: &'a Renderer, rp: &mut RenderPass<'b>)
        where 'a: 'b
    {
        rp.apply_material(renderer, &mut self.material, &self.camera);
        rp.draw_model(&self.model);
    }
}

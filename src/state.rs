use cgmath::{Vector3};
use wgpu::RenderPass;
use crate::camera::Camera;
use crate::input::Input;
use crate::material::{Material, MaterialParams, RenderMaterial};
use crate::model::{self, DrawModel, load_model};
use crate::renderer::Renderer;
use crate::texture::Texture;

pub struct State {
    pub material: Material,
    pub camera: Camera,
    model: model::Model,
}

impl State {
    pub async fn new(renderer: &Renderer) -> State {
        let texture = Texture::from_file("cube-diffuse.jpg", renderer).await.unwrap();
        let material = Material::diffuse(renderer, MaterialParams { texture }).await;

        let model = load_model("cube.obj", renderer).await.unwrap();

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

    pub fn render<'a, 'b>(&'a mut self, render_pass: &mut RenderPass<'b>, renderer: &'a Renderer)
        where 'a: 'b {
        render_pass.apply_material(renderer, &mut self.material, &self.camera);
        render_pass.draw_model(&self.model);
    }
}

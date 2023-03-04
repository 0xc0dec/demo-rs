use cgmath::{Vector3};
use wgpu::RenderPass;
use crate::camera::Camera;
use crate::input::Input;
use crate::material::{Material, MaterialParams, RenderMaterial};
use crate::model::{DrawModel, Model};
use crate::driver::Driver;
use crate::texture::Texture;

pub struct Scene {
    material: Material,
    camera: Camera,
    model: Model,
}

impl Scene {
    pub async fn new(device: &Driver) -> Scene {
        let texture = Texture::from_file("cube-diffuse.jpg", device).await.unwrap();
        let material = Material::diffuse(device, MaterialParams { texture }).await;

        let model = Model::from_file("cube.obj", device).await.expect("Failed to load cube model");

        let camera = Camera::new(
            Vector3::new(5.0, 5.0, 5.0),
            Vector3::new(0.0, 0.0, 0.0),
            device.canvas_size().into()
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

    pub fn render<'a, 'b>(&'a mut self, driver: &'a Driver, rp: &mut RenderPass<'b>)
        where 'a: 'b
    {
        rp.apply_material(driver, &mut self.material, &self.camera);
        rp.draw_model(&self.model);
    }
}

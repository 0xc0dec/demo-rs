use cgmath::{Vector3, Zero};
use wgpu::RenderPass;
use crate::camera::Camera;
use crate::driver::Driver;
use crate::input::Input;
use crate::materials::{DiffuseMaterial, DiffuseMaterialParams, Material, SkyboxMaterial, SkyboxMaterialParams};
use crate::model::{DrawModel, Mesh, Model};
use crate::scene::model_node::ModelNode;
use crate::scene::scene_node::SceneNode;
use crate::texture::Texture;
use crate::transform::Transform;

struct Skybox {
    mesh: Mesh,
    material: SkyboxMaterial,
}

pub struct Scene {
    camera: Camera,
    skybox: Skybox,
    models: Vec<Box<dyn SceneNode>>,
}

impl Scene {
    pub async fn new(driver: &Driver) -> Scene {
        let camera = Camera::new(
            Vector3::new(10.0, 10.0, 10.0),
            Vector3::new(0.0, 0.0, 0.0),
            driver.surface_size().into(),
        );

        let skybox_tex = Texture::from_file_cube("skybox_bgra.dds", driver).await.unwrap();

        Self {
            camera,
            skybox: Skybox {
                mesh: Mesh::quad(driver),
                material: SkyboxMaterial::new(driver, SkyboxMaterialParams { texture: skybox_tex }).await,
            },
            models: vec![
                Box::new(ModelNode {
                    model: Model::from_file("cube.obj", driver).await.expect("Failed to load cube model"),
                    transform: Transform::new(Vector3::zero()),
                    material: {
                        let texture = Texture::from_file_2d("stonewall.jpg", driver).await.unwrap();
                        DiffuseMaterial::new(driver, DiffuseMaterialParams { texture }).await
                    },
                }),
                // TODO Avoid duplicate loading
                Box::new(ModelNode {
                    model: Model::from_file("cube.obj", driver).await.expect("Failed to load cube model"),
                    transform: Transform::new(Vector3::unit_x() * 5.0),
                    material: {
                        let texture = Texture::from_file_2d("stonewall.jpg", driver).await.unwrap();
                        DiffuseMaterial::new(driver, DiffuseMaterialParams { texture }).await
                    },
                }),
            ],
        }
    }

    pub fn update(&mut self, input: &Input, dt: f32) {
        self.camera.update(input, dt);
        for n in &mut self.models {
            n.update(dt);
        }
    }

    pub fn render<'a, 'b>(&'a mut self, driver: &'a Driver, pass: &mut RenderPass<'b>)
        where 'a: 'b
    {
        self.skybox.material.update(&driver, &self.camera);
        self.skybox.material.apply(pass);
        pass.draw_mesh(&self.skybox.mesh);

        for m in &mut self.models {
            m.render(driver, &self.camera, pass);
        }
    }
}

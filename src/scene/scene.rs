use cgmath::{Vector3, Zero};
use rapier3d::prelude::*;
use wgpu::RenderPass;
use winit::dpi::PhysicalSize;
use crate::camera::Camera;
use crate::driver::Driver;
use crate::input::Input;
use crate::materials::{Material, SkyboxMaterial, SkyboxMaterialParams};
use crate::model::{DrawModel, Mesh};
use crate::physics::PhysicsWorld;
use crate::scene::model_node::ModelNode;
use crate::scene::scene_node::SceneNode;
use crate::texture::Texture;

struct Skybox {
    mesh: Mesh,
    material: SkyboxMaterial,
}

pub struct Scene {
    camera: Camera,
    skybox: Skybox,
    models: Vec<Box<dyn SceneNode>>,
    physics: PhysicsWorld,
}

impl Scene {
    pub async fn new(driver: &Driver) -> Scene {
        // let rigi/*d_body = RigidBodyBuilder::dynamic()
        //     .translation(vector![0.0, 10.0, 0.0])
        //     .build();
        // let collider = ColliderBuilder::ball(0.5).restitution(0.7).build();
        // let ball_body_handle = rigid_body_set.insert(rigid_body);
        // collider_set.insert_with_parent(collider, ball_body_handle, &mut rigid_body_set);

        let mut physics = PhysicsWorld::new();

        let obj1 = Box::new(ModelNode::new(Vector3::zero(), driver, &mut physics).await);
        let obj2 = Box::new(ModelNode::new(Vector3::unit_x() * 3.0, driver, &mut physics).await);
        let obj3 = Box::new(ModelNode::new(Vector3::unit_x() * -3.0, driver, &mut physics).await);

        let camera = Camera::new(
            Vector3::new(10.0, 10.0, 10.0),
            Vector3::new(0.0, 0.0, 0.0),
            driver.surface_size().into(),
        );

        let skybox_tex = Texture::from_file_cube("skybox_bgra.dds", driver).await.unwrap();

        Self {
            physics,
            camera,
            skybox: Skybox {
                mesh: Mesh::quad(driver),
                material: SkyboxMaterial::new(driver, SkyboxMaterialParams { texture: skybox_tex }).await,
            },
            models: vec![obj1, obj2, obj3],
        }
    }

    // TODO Remove this and instead react to resizing in update() or render()
    pub fn on_canvas_resize(&mut self, new_size: PhysicalSize<u32>) {
        self.camera.on_canvas_resize(new_size.into());
    }

    pub fn update(&mut self, input: &Input, dt: f32) {
        self.physics.update(dt);

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

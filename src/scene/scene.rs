use cgmath::{Vector3, Zero};
use rapier3d::prelude::*;
use wgpu::RenderPass;
use winit::dpi::PhysicalSize;
use crate::camera::Camera;
use crate::graphics::Graphics;
use crate::input::Input;
use crate::materials::{Material, SkyboxMaterial, SkyboxMaterialParams};
use crate::model::{DrawModel, Mesh};
use crate::physics::PhysicsWorld;
use crate::scene::test_node::{TestNode, TestNodeParams};
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
    pub async fn new(gfx: &Graphics) -> Scene {
        let mut physics = PhysicsWorld::new();

        let ground = Box::new(
            TestNode::new(
                gfx, &mut physics,
                TestNodeParams {
                    pos: Vector3::zero(),
                    scale: Vector3::new(10.0, 0.1, 10.0),
                    movable: false,
                }
            ).await
        );

        let box1 = Box::new(
            TestNode::new(
                gfx, &mut physics,
                TestNodeParams {
                    pos: Vector3::unit_y() * 10.0,
                    scale: Vector3::new(1.0, 1.0, 1.0),
                    movable: true,
                }
            ).await
        );

        let camera = Camera::new(
            Vector3::new(10.0, 10.0, 10.0),
            Vector3::new(0.0, 0.0, 0.0),
            gfx.surface_size().into(),
        );

        let skybox_tex = Texture::from_file_cube("skybox_bgra.dds", gfx).await.unwrap();

        Self {
            physics,
            camera,
            skybox: Skybox {
                mesh: Mesh::quad(gfx),
                material: SkyboxMaterial::new(gfx, SkyboxMaterialParams { texture: skybox_tex }).await,
            },
            models: vec![ground, box1],
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
            n.update(dt, &self.physics);
        }
    }

    pub fn render<'a, 'b>(&'a mut self, gfx: &'a Graphics, pass: &mut RenderPass<'b>)
        where 'a: 'b
    {
        self.skybox.material.update(&gfx, &self.camera);
        self.skybox.material.apply(pass);
        pass.draw_mesh(&self.skybox.mesh);

        for m in &mut self.models {
            m.render(gfx, &self.camera, pass);
        }
    }
}

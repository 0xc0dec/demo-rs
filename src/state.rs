use cgmath::{Vector3, Zero};
use wgpu::RenderPass;
use crate::camera::Camera;
use crate::graphics::Graphics;
use crate::events::Events;
use crate::materials::{Material, SkyboxMaterial, SkyboxMaterialParams};
use crate::model::{DrawModel, Mesh};
use crate::physics::PhysicsWorld;
use crate::scene::{Entity, TestEntity, TestEntityParams};
use crate::texture::Texture;

struct Skybox {
    mesh: Mesh,
    material: SkyboxMaterial,
}

pub struct State {
    camera: Camera,
    skybox: Skybox,
    entities: Vec<Box<dyn Entity>>,
    physics: PhysicsWorld,
}

impl State {
    pub async fn new(gfx: &Graphics) -> State {
        let mut physics = PhysicsWorld::new();

        let ground = Box::new(
            TestEntity::new(
                gfx, &mut physics,
                TestEntityParams {
                    pos: Vector3::zero(),
                    scale: Vector3::new(10.0, 0.1, 10.0),
                    movable: false,
                }
            ).await
        );

        let box1 = Box::new(
            TestEntity::new(
                gfx, &mut physics,
                TestEntityParams {
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
            entities: vec![ground, box1],
        }
    }

    pub fn update(&mut self, events: &Events, dt: f32) {
        self.physics.update(dt);
        self.camera.update(events, dt);
        for n in &mut self.entities {
            n.update(dt, &self.physics);
        }
    }

    pub fn render<'a, 'b>(&'a mut self, gfx: &'a Graphics, pass: &mut RenderPass<'b>, events: &'a Events)
        where 'a: 'b
    {
        if let Some(new_surface_size) = events.new_surface_size {
            self.camera.on_canvas_resize(new_surface_size)
        }

        self.skybox.material.update(&gfx, &self.camera);
        self.skybox.material.apply(pass);
        pass.draw_mesh(&self.skybox.mesh);

        for m in &mut self.entities {
            m.render(gfx, &self.camera, pass);
        }
    }
}

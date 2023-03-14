use cgmath::{Vector3, Zero};
use crate::camera::Camera;
use crate::device::{Device, Frame};
use crate::frame_context::FrameContext;
use crate::physics_world::PhysicsWorld;
use crate::scene::character::Character;
use crate::scene::skybox::Skybox;
use crate::scene::test_entity::{TestEntity, TestEntityParams};
use crate::scene::tracer::Tracer;

pub struct Scene {
    character: Character,
    tracer: Tracer,
    skybox: Skybox,
    entities: Vec<TestEntity>,
    physics: PhysicsWorld,
}

impl Scene {
    pub async fn new(device: &Device) -> Scene {
        let mut physics = PhysicsWorld::new();

        let ground = TestEntity::new(
            device,
            &mut physics,
            TestEntityParams {
                pos: Vector3::zero(),
                scale: Vector3::new(10.0, 0.1, 10.0),
                movable: false,
            }
        ).await;

        let box1 = TestEntity::new(
            device,
            &mut physics,
            TestEntityParams {
                pos: Vector3::unit_y() * 10.0,
                scale: Vector3::new(1.0, 1.0, 1.0),
                movable: true,
            }
        ).await;

        let character = Character::new(
            Camera::new(
                Vector3::new(10.0, 10.0, 10.0),
                Vector3::new(0.0, 0.0, 0.0),
                device.surface_size().into(),
            ),
            &mut physics
        );

        let tracer = Tracer::new(device).await;
        let skybox = Skybox::new(device).await;

        Self {
            physics,
            character,
            tracer,
            skybox,
            entities: vec![ground, box1]
        }
    }

    pub fn update(&mut self, ctx: &FrameContext) {
        self.physics.update(ctx.dt);

        self.character.update(ctx, &mut self.physics);
        self.tracer.update(&self.physics, &self.character);

        for e in &mut self.entities {
            e.update(ctx.dt, &self.physics);
        }
    }

    pub fn render<'a, 'b>(&'a mut self, frame: &mut Frame<'b, 'a>, ctx: &'a FrameContext)
        where 'a: 'b
    {
        // TODO Do this only when the size changes
        self.character.camera.set_fov(
            ctx.device.surface_size().width as f32,
            ctx.device.surface_size().height as f32
        );

        self.skybox.render(ctx.device, &self.character.camera, frame);

        for e in &mut self.entities {
            e.render(ctx.device, &self.character.camera, frame);
        }

        self.tracer.render(ctx.device, &self.character.camera, frame);
    }
}

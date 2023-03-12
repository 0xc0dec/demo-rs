use cgmath::{Vector3, Zero};
use crate::camera::Camera;
use crate::device::{Device, Frame};
use crate::frame_context::FrameContext;
use crate::physics_world::PhysicsWorld;
use crate::scene::character::Character;
use crate::scene::entity::Entity;
use crate::scene::skybox::Skybox;
use crate::scene::test_entity::{TestEntity, TestEntityParams};

pub struct Scene {
    character: Character,
    skybox: Skybox,
    entities: Vec<Box<dyn Entity>>,
    physics: PhysicsWorld,
}

impl Scene {
    pub async fn new(device: &Device) -> Scene {
        let mut physics = PhysicsWorld::new();

        let ground = Box::new(
            TestEntity::new(
                device,
                &mut physics,
                TestEntityParams {
                    pos: Vector3::zero(),
                    scale: Vector3::new(10.0, 0.1, 10.0),
                    movable: false,
                }
            ).await
        );

        let box1 = Box::new(
            TestEntity::new(
                device,
                &mut physics,
                TestEntityParams {
                    pos: Vector3::unit_y() * 10.0,
                    scale: Vector3::new(1.0, 1.0, 1.0),
                    movable: true,
                }
            ).await
        );

        let character = Character::new(
            Camera::new(
                Vector3::new(10.0, 10.0, 10.0),
                Vector3::new(0.0, 0.0, 0.0),
                device.surface_size().into(),
            ),
            &mut physics
        );

        Self {
            physics,
            character,
            skybox: Skybox::new(device).await,
            entities: vec![ground, box1]
        }
    }

    pub fn update(&mut self, ctx: &FrameContext) {
        self.physics.update(ctx.dt);
        self.character.update(ctx, &mut self.physics);
        for n in &mut self.entities {
            n.update(ctx.dt, &self.physics);
        }
    }

    pub fn render<'a, 'b>(&'a mut self, device: &'a Device, frame: &mut Frame<'b, 'a>)
        where 'a: 'b
    {
        // TODO Do this only when the size changes
        self.character.camera
            .set_fov(device.surface_size().width as f32, device.surface_size().height as f32);

        self.skybox.render(device, &self.character.camera, frame);

        for m in &mut self.entities {
            m.render(device, &self.character.camera, frame);
        }
    }
}

use cgmath::{Vector3, Zero};
use crate::camera::Camera;
use crate::device::{Device, Frame};
use crate::frame_context::FrameContext;
use crate::model::{DrawModel, Mesh};
use crate::physics::PhysicsWorld;
use crate::scene::{Entity, TestEntity, TestEntityParams};
use crate::shaders::{Shader, SkyboxShader, SkyboxShaderParams};
use crate::texture::Texture;

struct Skybox {
    mesh: Mesh,
    shader: SkyboxShader,
}

pub struct State {
    camera: Camera,
    skybox: Skybox,
    entities: Vec<Box<dyn Entity>>,
    physics: PhysicsWorld,
}

impl State {
    pub async fn new(gfx: &Device) -> State {
        let mut physics = PhysicsWorld::new();

        let ground = Box::new(
            TestEntity::new(
                gfx,
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
                gfx,
                &mut physics,
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

        let skybox_tex = Texture::new_cube_from_file("skybox_bgra.dds", gfx).await.unwrap();

        Self {
            physics,
            camera,
            skybox: Skybox {
                mesh: Mesh::quad(gfx),
                shader: SkyboxShader::new(gfx, SkyboxShaderParams { texture: skybox_tex }).await,
            },
            entities: vec![ground, box1]
        }
    }

    pub fn update(&mut self, ctx: &FrameContext) {
        self.physics.update(ctx.dt);
        self.camera.update(ctx.events, ctx.dt);
        for n in &mut self.entities {
            n.update(ctx.dt, &self.physics);
        }
    }

    pub fn render<'a, 'b>(&'a mut self, device: &'a Device, frame: &mut Frame<'b, 'a>)
        where 'a: 'b
    {
        // TODO Do this only when the size changes
        self.camera.set_fov(device.surface_size().width as f32, device.surface_size().height as f32);

        self.skybox.shader.update(&device, &self.camera);
        self.skybox.shader.apply(frame);
        frame.draw_mesh(&self.skybox.mesh);

        for m in &mut self.entities {
            m.render(device, &self.camera, frame);
        }
    }
}

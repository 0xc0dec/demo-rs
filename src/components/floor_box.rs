use crate::components::model_renderer::ModelShader;
use crate::components::transform::Transform;
use crate::components::{ModelRenderer, PhysicsBody, PhysicsBodyParams};
use crate::device::Device;
use crate::math::Vec3;
use crate::model::Model;
use crate::physics_world::PhysicsWorld;
use crate::render_tags::RenderTags;
use crate::shaders::{DiffuseShader, DiffuseShaderParams};
use crate::texture::Texture;
use bevy_ecs::prelude::*;

#[derive(Component)]
pub struct FloorBox;

impl FloorBox {
    pub fn spawn(
        mut commands: Commands,
        device: NonSend<Device>,
        mut physics: NonSendMut<PhysicsWorld>,
    ) {
        let (shader, model) = pollster::block_on(async {
            let texture = Texture::new_2d_from_file("stonewall.jpg", &device)
                .await
                .unwrap();
            let shader =
                DiffuseShader::new(&device, DiffuseShaderParams { texture: &texture }).await;
            let model = Model::from_file("cube.obj", &device).await.unwrap();
            (shader, model)
        });

        let render_model = ModelRenderer {
            shader: ModelShader::Diffuse(shader),
            model,
            tags: RenderTags::SCENE,
        };

        let pos = Vec3::from_element(0.0);
        let scale = Vec3::new(10.0, 0.5, 10.0);
        let transform = Transform::new(pos, scale);

        let physics_body = PhysicsBody::new(
            PhysicsBodyParams {
                pos,
                scale,
                rotation_axis: Vec3::from_element(0.0),
                rotation_angle: 0.0,
                movable: false,
            },
            &mut physics,
        );

        commands.spawn((FloorBox, physics_body, render_model, transform));
    }
}

use bevy_ecs::prelude::*;
use crate::components::{ModelRenderer, ModelShader, PhysicsBody, PhysicsBodyParams, RenderLayer};
use crate::device::Device;
use crate::math::Vec3;
use crate::model::Model;
use crate::physics_world::PhysicsWorld;
use crate::shaders::{DiffuseShader, DiffuseShaderParams};
use crate::texture::Texture;
use crate::components::transform::Transform;

#[derive(Component)]
pub struct FreeBox;

impl FreeBox {
    pub fn spawn(mut commands: Commands, device: NonSend<Device>, mut physics: NonSendMut<PhysicsWorld>) {
        pollster::block_on(async {
            let pos = Vec3::y_axis().xyz() * 15.0;
            let scale = Vec3::from_element(1.0);

            let physics_body = PhysicsBody::new(
                PhysicsBodyParams {
                    pos,
                    scale,
                    rotation_axis: Vec3::from_element(1.0),
                    rotation_angle: 50.0,
                    movable: true,
                },
                &mut physics,
            );

            let texture = Texture::new_2d_from_file("stonewall.jpg", &device).await.unwrap();
            let shader = DiffuseShader::new(
                &device,
                DiffuseShaderParams {
                    texture: &texture
                },
            ).await;
            let model = Model::from_file("cube.obj", &device).await.unwrap();
            let render_model = ModelRenderer {
                shader: ModelShader::Diffuse(shader),
                model,
            };

            let transform = Transform::new(pos, scale);

            commands.spawn((
                FreeBox,
                RenderLayer(100),
                physics_body,
                render_model,
                transform
            ));
        });
    }
}
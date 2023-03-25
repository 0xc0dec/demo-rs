use bevy_ecs::prelude::*;
use rapier3d::prelude::*;
use wgpu::{RenderBundleEncoder, RenderPass};
use crate::components::{Camera, RenderLayer, RenderModel};
use crate::components::render_model::ModelShader;
use crate::device::Device;
use crate::math::Vec3;
use crate::model::{DrawModel, Model};
use crate::physics_world::PhysicsWorld;
use crate::shaders::{DiffuseShader, DiffuseShaderParams, Shader};
use crate::texture::Texture;
use crate::transform::Transform;

#[derive(Component)]
pub struct PhysicsBox {
    rigid_body_handle: RigidBodyHandle,
}

#[derive(Copy, Clone)]
pub struct PhysicsBoxParams {
    pub pos: Vec3,
    pub scale: Vec3,
    pub rotation_angle: f32,
    pub rotation_axis: Vec3,
    pub movable: bool,
}

impl PhysicsBox {
    pub fn spawn(params: PhysicsBoxParams) -> impl Fn(Commands, NonSend<Device>, NonSendMut<PhysicsWorld>) {
        move |mut commands: Commands, device: NonSend<Device>, mut physics: NonSendMut<PhysicsWorld>| {
            pollster::block_on(async {
                let PhysicsBoxParams {
                    pos,
                    scale,
                    rotation_axis,
                    rotation_angle,
                    movable,
                } = params;

                let body = if movable { RigidBodyBuilder::dynamic() } else { RigidBodyBuilder::fixed() }
                    .translation(vector![pos.x, pos.y, pos.z])
                    // TODO Verify this conversion
                    .rotation(rotation_axis * rotation_angle)
                    .build();
                let collider = ColliderBuilder::cuboid(scale.x, scale.y, scale.z)
                    .restitution(0.2)
                    .friction(0.7)
                    .build();
                let (rigid_body_handle, _) = physics.add_body(body, collider);

                let transform = Transform::new(pos, scale);
                let texture = Texture::new_2d_from_file("stonewall.jpg", &device).await.unwrap();
                let shader = DiffuseShader::new(
                    &device,
                    DiffuseShaderParams {
                        texture: &texture
                    }
                ).await;
                let model = Model::from_file("cube.obj", &device).await.unwrap();
                let render_model = RenderModel {
                    shader: ModelShader::Diffuse(shader),
                    model,
                    transform
                };

                commands.spawn((
                    Self {
                        rigid_body_handle
                    },
                    render_model,
                    RenderLayer(100)
                ));
            });
        }
    }
}
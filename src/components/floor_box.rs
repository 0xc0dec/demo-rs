use bevy_ecs::prelude::*;

use crate::assets::Assets;
use crate::components::mesh_renderer::Material;
use crate::components::render_tags::RenderTags;
use crate::components::transform::Transform;
use crate::components::{MeshRenderer, PhysicsBody, PhysicsBodyParams};
use crate::device::Device;
use crate::math::Vec3;
use crate::mesh::Mesh;
use crate::physics_world::PhysicsWorld;
use crate::render_tags::RENDER_TAG_SCENE;
use crate::shaders::DiffuseShader;

#[derive(Component)]
pub struct FloorBox;

impl FloorBox {
    pub fn spawn(
        mut commands: Commands,
        device: Res<Device>,
        mut physics: ResMut<PhysicsWorld>,
        assets: Res<Assets>,
    ) {
        let (shader, mesh) = pollster::block_on(async {
            let shader = DiffuseShader::new(&device, &assets, &assets.stone_tex);
            let mesh = Mesh::from_file("cube.obj", &device).await;
            (shader, mesh)
        });

        let renderer = MeshRenderer::new(mesh, Material::Diffuse(shader));
        let pos = Vec3::from_element(0.0);
        let scale = Vec3::new(10.0, 0.5, 10.0);
        let transform = Transform::new(pos, scale);
        let body = PhysicsBody::new(
            PhysicsBodyParams {
                pos,
                scale,
                rotation_axis: Vec3::from_element(0.0),
                rotation_angle: 0.0,
                movable: false,
            },
            &mut physics,
        );

        commands.spawn((
            FloorBox,
            body,
            renderer,
            transform,
            RenderTags(RENDER_TAG_SCENE),
        ));
    }
}

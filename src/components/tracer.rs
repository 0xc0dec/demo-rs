use crate::components::{MeshRenderer, ShaderVariant, Player, Transform};
use crate::device::Device;
use crate::math::Vec3;
use crate::mesh::Mesh;
use crate::physics_world::PhysicsWorld;
use crate::render_tags::RenderTags;
use crate::shaders::ColorShader;
use bevy_ecs::prelude::*;

#[derive(Component)]
pub struct Tracer;

impl Tracer {
    pub fn spawn(mut commands: Commands, device: NonSend<Device>) {
        let transform = Transform::default();

        let (mesh, shader) = pollster::block_on(async {
            let mesh = Mesh::from_file("cube.obj", &device).await;
            let shader = ColorShader::new(&device).await;
            (mesh, shader)
        });

        let renderer = MeshRenderer::new(
            mesh,
            ShaderVariant::Color(shader),
            RenderTags::HIDDEN,
        );

        commands.spawn((Tracer, transform, renderer));
    }

    pub fn update(
        physics: Res<PhysicsWorld>,
        // Without this Without it crashes :|
        player: Query<(&Player, &Transform), Without<Tracer>>,
        mut tracer: Query<(&mut Transform, &mut MeshRenderer), With<Tracer>>,
    ) {
        let (player, player_transform) = player.single();
        let (mut tracer_transform, mut tracer_renderer) = tracer.single_mut();

        if let Some((hit_pt, _, _)) = physics.cast_ray(
            player_transform.position(),
            player_transform.forward(),
            Some(player.collider_handle()),
        ) {
            let dist_to_camera = (player_transform.position() - hit_pt).magnitude();
            let scale = (dist_to_camera / 10.0).min(0.1).max(0.01);

            tracer_renderer.set_tags(RenderTags::SCENE);
            tracer_transform.set_position(hit_pt);
            tracer_transform.set_scale(Vec3::from_element(scale));
        } else {
            tracer_renderer.set_tags(RenderTags::HIDDEN);
        }
    }
}

use bevy_ecs::prelude::*;

use crate::assets::Assets;
use crate::components::render_tags::RenderTags;
use crate::components::{Camera, Material, MeshRenderer, Player, RenderOrder, Transform};
use crate::device::Device;
use crate::mesh::Mesh;
use crate::render_tags::{RENDER_TAG_DEBUG_UI, RENDER_TAG_POST_PROCESS};
use crate::shaders::PostProcessShader;
use crate::texture::TextureSize;

#[derive(Component)]
pub struct PostProcessor {
    size: TextureSize,
}

impl PostProcessor {
    pub fn spawn(
        mut commands: Commands,
        player: Query<&Camera, With<Player>>,
        device: Res<Device>,
        assets: Res<Assets>,
    ) {
        // We know we need the player camera
        let source_camera_rt = player.single().target().as_ref().unwrap();
        let mesh = Mesh::quad(&device);
        let shader = PostProcessShader::new(&device, &assets, source_camera_rt.color_tex());
        let renderer = MeshRenderer::new(mesh, Material::PostProcess(shader));
        let transform = Transform::default();
        let pp = PostProcessor {
            size: source_camera_rt.color_tex().size(),
        };

        commands.spawn((renderer, transform, pp, RenderTags(RENDER_TAG_POST_PROCESS)));

        // Camera for rendering the quad (and debug UI for that matter)
        let camera = Camera::new(1.0, RENDER_TAG_POST_PROCESS | RENDER_TAG_DEBUG_UI, None);
        let transform = Transform::default();
        commands.spawn((RenderOrder(100), camera, transform));
    }

    pub fn update(
        device: Res<Device>,
        mut pp: Query<(&PostProcessor, &mut MeshRenderer)>,
        player_cam: Query<&Camera, With<Player>>,
        assets: Res<Assets>,
    ) {
        if let Some(pp) = pp.iter_mut().next().as_mut() {
            let source_camera_rt = player_cam.single().target().as_ref().unwrap();

            if source_camera_rt.color_tex().size() != pp.0.size {
                // TODO Better. We should NOT be re-creating the shader.
                pp.1.material = Material::PostProcess(PostProcessShader::new(
                    &device,
                    &assets,
                    source_camera_rt.color_tex(),
                ));
            }
        }
    }
}

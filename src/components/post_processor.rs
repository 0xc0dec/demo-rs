use bevy_ecs::prelude::{Commands, NonSend, Query};
use crate::components::{Camera, ModelRenderer, ModelShader, Player, RenderOrder, Transform};
use crate::device::Device;
use crate::model::Model;
use crate::render_tags::RenderTags;
use crate::shaders::{PostProcessShader, PostProcessShaderParams};

pub struct PostProcessor;

impl PostProcessor {
    pub fn spawn(
        mut commands: Commands,
        player: Query<(&Player, &Camera)>,
        device: NonSend<Device>,
    ) {
        // We know we need the player camera
        let source_camera_rt = player.single().1.target().as_ref().unwrap();

        let model = Model::quad(&device);

        // TODO Refactor similar places - use blocking only on the async pieces of code
        let shader = pollster::block_on(async {
            PostProcessShader::new(
                &device,
                PostProcessShaderParams {
                    texture: source_camera_rt.color_tex(),
                },
            ).await
        });

        let model_renderer = ModelRenderer {
            shader: ModelShader::PostProcess(shader),
            model,
            tags: RenderTags::POST_PROCESS,
        };
        let transform = Transform::default();
        commands.spawn((model_renderer, transform));

        let camera = Camera::new(1.0, RenderTags::POST_PROCESS, None);
        let transform = Transform::default();
        commands.spawn((RenderOrder(100), camera, transform));
    }
}
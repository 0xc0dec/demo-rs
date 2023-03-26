use bevy_ecs::prelude::{Commands, NonSend};
use crate::components::{Camera, ModelRenderer, ModelShader, RenderOrder, Transform};
use crate::device::Device;
use crate::model::Model;
use crate::render_tags::RenderTags;
use crate::render_target::RenderTarget;
use crate::shaders::{PostProcessShader, PostProcessShaderParams};

pub struct PostProcessor;

impl PostProcessor {
    pub fn spawn(mut commands: Commands, device: NonSend<Device>) {
        pollster::block_on(async {
            let rt = RenderTarget::new(&device, None);
            let shader = PostProcessShader::new(
                &device,
                PostProcessShaderParams {
                    texture: rt.color_tex(),
                },
            )
                .await;
            let model = Model::quad(&device);
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
        });
    }
}
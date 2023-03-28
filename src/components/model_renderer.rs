use crate::components::transform::Transform;
use crate::components::Camera;
use crate::device::Device;
use crate::mesh::{DrawModel, Model};
use crate::shaders::{ColorShader, DiffuseShader, PostProcessShader, Shader, SkyboxShader};
use bevy_ecs::prelude::*;

pub enum ModelShader {
    Color(ColorShader),
    Diffuse(DiffuseShader),
    Skybox(SkyboxShader),
    PostProcess(PostProcessShader),
}

#[derive(Component)]
pub struct ModelRenderer {
    model: Model,
    shader: ModelShader,
    tags: u32, // TODO As a component?
}

impl ModelRenderer {
    pub fn new(model: Model, shader: ModelShader, tags: u32) -> ModelRenderer {
        Self {
            model,
            shader,
            tags
        }
    }

    pub fn render<'a>(
        &'a mut self,
        device: &Device,
        camera: (&Camera, &Transform),
        transform: &Transform,
        encoder: &mut wgpu::RenderBundleEncoder<'a>,
    ) {
        // TODO Generalize
        match self.shader {
            ModelShader::Color(ref mut color) => {
                color.update(device, camera, &transform);
                color.apply(encoder);
            }
            ModelShader::Diffuse(ref mut diffuse) => {
                diffuse.update(device, camera, &transform);
                diffuse.apply(encoder);
            }
            ModelShader::Skybox(ref mut skybox) => {
                skybox.update(device, camera);
                skybox.apply(encoder);
            }
            ModelShader::PostProcess(ref mut pp) => {
                pp.apply(encoder);
            }
        }
        encoder.draw_model(&self.model);
    }

    pub fn tags(&self) -> u32 {
        self.tags
    }

    pub fn set_tags(&mut self, tags: u32) {
        self.tags = tags;
    }

    pub fn set_shader(&mut self, shader: ModelShader) {
        self.shader = shader;
    }
}

use bevy_ecs::prelude::Component;
use crate::components::Camera;
use crate::device::Device;
use crate::model::{DrawModel, Model};
use crate::shaders::{ColorShader, DiffuseShader, Shader};
use crate::transform::Transform;

pub enum ModelShader {
    Color(ColorShader),
    Diffuse(DiffuseShader)
}

#[derive(Component)]
pub struct RenderModel {
    pub model: Model,
    pub transform: Transform,
    pub shader: ModelShader,
}

impl RenderModel {
    pub fn render<'a>(
        &'a mut self,
        device: &Device,
        camera: &Camera,
        encoder: &mut wgpu::RenderBundleEncoder<'a>,
    ) {
        // TODO Generalize
        match self.shader {
            ModelShader::Color(ref mut color) => {
                color.update(device, camera, &self.transform);
                color.apply(encoder);
            }
            ModelShader::Diffuse(ref mut diffuse) => {
                diffuse.update(device, camera, &self.transform);
                diffuse.apply(encoder);
            }
        }
        encoder.draw_model(&self.model);
    }
}
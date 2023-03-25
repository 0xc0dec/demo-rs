use bevy_ecs::prelude::Component;
use crate::model::Model;
use crate::shaders::{ColorShader, DiffuseShader};
use crate::transform::Transform;

pub enum ModelShader {
    Color(ColorShader),
    Diffuse(DiffuseShader)
}

impl ModelShader {}

#[derive(Component)]
pub struct RenderModel {
    pub model: Model,
    pub transform: Transform,
    pub shader: ModelShader,
}

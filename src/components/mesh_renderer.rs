use bevy_ecs::prelude::*;

use crate::components::transform::Transform;
use crate::components::Camera;
use crate::device::Device;
use crate::mesh::{DrawMesh, Mesh};
use crate::shaders::{ColorShader, DiffuseShader, PostProcessShader, SkyboxShader};

pub enum ShaderVariant {
    Color(ColorShader),
    Diffuse(DiffuseShader),
    Skybox(SkyboxShader),
    PostProcess(PostProcessShader),
}

/**
Plan:
- Remove MeshRenderer, switch the code that uses it to use Mesh + Material pair.
- Make shader lightweight and load it as a resource.
- Uniforms should be part of a shader definition?
- Make tags a component.
 */
#[derive(Component)]
pub struct MeshRenderer {
    pub tags: u32,
    // TODO As a component?
    mesh: Mesh,
    shader: ShaderVariant,
}

impl MeshRenderer {
    pub fn new(mesh: Mesh, shader: ShaderVariant, tags: u32) -> MeshRenderer {
        Self { mesh, shader, tags }
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
            ShaderVariant::Color(ref mut color) => {
                color.update_uniforms(device, camera, transform);
                color.apply(encoder);
            }
            ShaderVariant::Diffuse(ref mut diffuse) => {
                diffuse.update_uniforms(device, camera, transform);
                diffuse.apply(encoder);
            }
            ShaderVariant::Skybox(ref mut skybox) => {
                skybox.update_uniforms(device, camera);
                skybox.apply(encoder);
            }
            ShaderVariant::PostProcess(ref mut pp) => {
                pp.apply(encoder);
            }
        }
        encoder.draw_mesh(&self.mesh);
    }

    pub fn set_shader(&mut self, shader: ShaderVariant) {
        self.shader = shader;
    }
}

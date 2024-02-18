use bevy_ecs::prelude::*;

use crate::components::transform::Transform;
use crate::components::Camera;
use crate::device::Device;
use crate::materials::{ColorMaterial, DiffuseMaterial, PostProcessMaterial, SkyboxMaterial};
use crate::mesh::{DrawMesh, Mesh};

pub enum Material {
    Color(ColorMaterial),
    Diffuse(DiffuseMaterial),
    Skybox(SkyboxMaterial),
    PostProcess(PostProcessMaterial),
}

/**
Plan:
- Remove MeshRenderer, switch the code that uses it to use Mesh + Material pair.
- Make shader lightweight and load it as a resource.
- Uniforms should be part of a shader definition?
 */
#[derive(Component)]
pub struct MeshRenderer {
    // TODO As a component?
    mesh: Mesh,
    pub material: Material,
}

impl MeshRenderer {
    pub fn new(mesh: Mesh, material: Material) -> MeshRenderer {
        Self { mesh, material }
    }

    pub fn render<'a>(
        &'a mut self,
        device: &Device,
        camera: (&Camera, &Transform),
        transform: &Transform,
        encoder: &mut wgpu::RenderBundleEncoder<'a>,
    ) {
        // TODO Generalize
        match self.material {
            Material::Color(ref mut color) => {
                color.update_uniforms(device, camera, transform);
                color.apply(encoder);
            }
            Material::Diffuse(ref mut diffuse) => {
                diffuse.update_uniforms(device, camera, transform);
                diffuse.apply(encoder);
            }
            Material::Skybox(ref mut skybox) => {
                skybox.update_uniforms(device, camera);
                skybox.apply(encoder);
            }
            Material::PostProcess(ref mut pp) => {
                pp.apply(encoder);
            }
        }
        encoder.draw_mesh(&self.mesh);
    }
}

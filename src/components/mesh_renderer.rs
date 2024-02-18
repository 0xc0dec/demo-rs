use bevy_ecs::prelude::*;

use crate::assets::{DrawMesh, Mesh};

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
}

impl MeshRenderer {
    pub fn new(mesh: Mesh) -> MeshRenderer {
        Self { mesh }
    }

    pub fn render<'a>(&'a self, encoder: &mut wgpu::RenderBundleEncoder<'a>) {
        encoder.draw_mesh(&self.mesh);
    }
}

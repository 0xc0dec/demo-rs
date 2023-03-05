mod diffuse_material;
mod skybox_material;

pub use diffuse_material::*;
pub use skybox_material::*;

pub trait Material {
    fn apply<'a, 'b>(&'a mut self, pass: &mut wgpu::RenderPass<'b>) where 'a: 'b;
}
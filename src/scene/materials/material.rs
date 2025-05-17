use super::{ColorMaterial, PostProcessMaterial, SkyboxMaterial, TexturedMaterial};

pub enum Material {
    Color(ColorMaterial),
    Skybox(SkyboxMaterial),
    Textured(TexturedMaterial),
    PostProcess(PostProcessMaterial),
}

impl Material {
    pub fn apply<'a>(&'a self, encoder: &mut wgpu::RenderBundleEncoder<'a>) {
        match self {
            Material::Color(m) => m.apply(encoder),
            Material::Skybox(m) => m.apply(encoder),
            Material::Textured(m) => m.apply(encoder),
            Material::PostProcess(m) => m.apply(encoder),
        };
    }
}

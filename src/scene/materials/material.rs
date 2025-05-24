use super::super::components::{Camera, Transform};
use super::{ColorMaterial, PostProcessMaterial, SkyboxMaterial, TexturedMaterial};
use crate::render;
use crate::render::Renderer;
use crate::scene::Assets;
use wgpu::RenderBundleEncoder;

// TODO Avoid this crap, via trait objects or smth
pub enum Material {
    Color(ColorMaterial),
    Skybox(SkyboxMaterial),
    Textured(TexturedMaterial),
    PostProcess(PostProcessMaterial),
}

impl Material {
    pub fn textured(rr: &Renderer, assets: &mut Assets, tex_path: &str) -> Material {
        let shader = assets.add_shader_from_file(rr, "textured.wgsl");
        let tex = assets.add_2d_texture_from_file(rr, tex_path);
        // TODO We shouldn't call assets again to get the actual objects, they should be returned
        // from the Assets' methods that created them.2
        Material::Textured(TexturedMaterial::new(
            rr,
            assets.shader(shader),
            assets.texture(tex),
        ))
    }

    pub fn update(&self, rr: &Renderer, cam: &Camera, cam_tr: &Transform, tr: &Transform) {
        match self {
            Material::Color(m) => m.set_wvp(rr, cam, cam_tr, tr),
            Material::Textured(m) => m.set_wvp(rr, cam, cam_tr, tr),
            Material::Skybox(m) => m.set_wvp(rr, cam, cam_tr),
            Material::PostProcess(_) => (),
        }
    }
}

impl render::ApplyMaterial for Material {
    fn apply<'a>(&'a self, encoder: &mut RenderBundleEncoder<'a>) {
        match self {
            Material::Color(m) => m.apply(encoder),
            Material::Skybox(m) => m.apply(encoder),
            Material::Textured(m) => m.apply(encoder),
            Material::PostProcess(m) => m.apply(encoder),
        };
    }
}

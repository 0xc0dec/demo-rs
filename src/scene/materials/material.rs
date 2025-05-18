use crate::render;
use crate::render::Renderer;
use wgpu::RenderBundleEncoder;

use super::super::components::{Camera, Transform};
use super::{ColorMaterial, PostProcessMaterial, SkyboxMaterial, TexturedMaterial};

pub enum Material {
    Color(ColorMaterial),
    Skybox(SkyboxMaterial),
    Textured(TexturedMaterial),
    PostProcess(PostProcessMaterial),
}

impl Material {
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

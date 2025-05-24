use super::super::components::{Camera, Transform};
use super::super::Assets;
use super::color::ColorMaterial;
use super::post_process::PostProcessMaterial;
use super::skybox::SkyboxMaterial;
use super::textured::TexturedMaterial;
use crate::math::Vec3;
use crate::render;
use crate::render::{Renderer, Texture};

// TODO Avoid this crap, use trait objects or smth
pub enum Material {
    Color(ColorMaterial),
    Skybox(SkyboxMaterial),
    Textured(TexturedMaterial),
    PostProcess(PostProcessMaterial),
}

impl Material {
    pub fn textured(rr: &Renderer, assets: &mut Assets, tex_path: &str) -> Self {
        let shader = assets.add_shader_from_file(rr, "textured.wgsl");
        let tex = assets.add_2d_texture_from_file(rr, tex_path);
        // TODO We shouldn't call assets again to get the actual objects, they should be returned
        // from the Assets' methods that created them.
        Self::Textured(TexturedMaterial::new(
            rr,
            assets.shader(shader),
            assets.texture(tex),
        ))
    }

    pub fn post_process(rr: &Renderer, assets: &mut Assets, src_texture: &Texture) -> Self {
        let shader = assets.add_shader_from_file(rr, "post-process.wgsl");
        // TODO We shouldn't call assets again to get the actual objects, they should be returned
        // from the Assets' methods that created them.
        Self::PostProcess(PostProcessMaterial::new(
            rr,
            assets.shader(shader),
            src_texture,
        ))
    }

    pub fn skybox(rr: &Renderer, assets: &mut Assets, tex_path: &str) -> Self {
        let shader = assets.add_shader_from_file(rr, "skybox.wgsl");
        let tex = assets.add_cube_texture_from_file(rr, tex_path);
        // TODO We shouldn't call assets again to get the actual objects, they should be returned
        // from the Assets' methods that created them.
        Self::Skybox(SkyboxMaterial::new(
            rr,
            assets.shader(shader),
            assets.texture(tex),
        ))
    }

    pub fn color(rr: &Renderer, assets: &mut Assets, color: Vec3, wireframe: bool) -> Self {
        let shader = assets.add_shader_from_file(rr, "color.wgsl");
        // TODO We shouldn't call assets again to get the actual objects, they should be returned
        // from the Assets' methods that created them.
        Self::Color(ColorMaterial::new(
            rr,
            assets.shader(shader),
            color,
            wireframe,
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
    fn apply<'a>(&'a self, encoder: &mut wgpu::RenderBundleEncoder<'a>) {
        match self {
            Material::Color(m) => m.apply(encoder),
            Material::Skybox(m) => m.apply(encoder),
            Material::Textured(m) => m.apply(encoder),
            Material::PostProcess(m) => m.apply(encoder),
        };
    }
}

use crate::assets::Assets;
use crate::graphics::Graphics;
use crate::texture::Texture;
use crate::transform::Transform;

use super::apply_material::ApplyMaterial;
use super::color::ColorMaterial;
use super::diffuse::DiffuseMaterial;
use super::post_process::PostProcessMaterial;
use super::skybox::SkyboxMaterial;
use super::super::Camera;

pub struct Material(Box<dyn ApplyMaterial>);

impl ApplyMaterial for Material {
    fn apply<'a>(
        &'a mut self,
        encoder: &mut wgpu::RenderBundleEncoder<'a>,
        gfx: &Graphics,
        camera: (&Camera, &Transform),
        transform: &Transform,
    ) {
        self.0.apply(encoder, gfx, camera, transform);
    }
}

impl Material {
    pub fn color(gfx: &Graphics, assets: &Assets) -> Self {
        Self(Box::new(ColorMaterial::new(gfx, assets)))
    }

    pub fn diffuse(gfx: &Graphics, assets: &Assets, texture: &Texture) -> Self {
        Self(Box::new(DiffuseMaterial::new(gfx, assets, texture)))
    }

    pub fn skybox(gfx: &Graphics, assets: &Assets, texture: &Texture) -> Self {
        Self(Box::new(SkyboxMaterial::new(gfx, assets, texture)))
    }

    pub fn post_process(gfx: &Graphics, assets: &Assets, texture: &Texture) -> Self {
        Self(Box::new(PostProcessMaterial::new(gfx, assets, texture)))
    }
}

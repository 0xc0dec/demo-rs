use crate::assets::{Assets, Texture};
use crate::device::Device;

use super::super::{Camera, Transform};
use super::apply_material::ApplyMaterial;
use super::color::ColorMaterial;
use super::diffuse::DiffuseMaterial;
use super::post_process::PostProcessMaterial;
use super::skybox::SkyboxMaterial;

pub struct Material(Box<dyn ApplyMaterial>);

impl ApplyMaterial for Material {
    fn apply<'a>(
        &'a mut self,
        encoder: &mut wgpu::RenderBundleEncoder<'a>,
        device: &Device,
        camera: (&Camera, &Transform),
        transform: &Transform,
    ) {
        self.0.apply(encoder, device, camera, transform);
    }
}

impl Material {
    pub fn color(device: &Device, assets: &Assets) -> Self {
        Self(Box::new(ColorMaterial::new(device, assets)))
    }

    pub fn diffuse(device: &Device, assets: &Assets, texture: &Texture) -> Self {
        Self(Box::new(DiffuseMaterial::new(device, assets, texture)))
    }

    pub fn skybox(device: &Device, assets: &Assets, texture: &Texture) -> Self {
        Self(Box::new(SkyboxMaterial::new(device, assets, texture)))
    }

    pub fn post_process(device: &Device, assets: &Assets, texture: &Texture) -> Self {
        Self(Box::new(PostProcessMaterial::new(device, assets, texture)))
    }
}

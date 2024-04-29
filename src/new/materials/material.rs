use crate::new::{Assets, Device, Texture};
use crate::new::camera::Camera;
use crate::new::transform::Transform;

use super::ApplyMaterial;
use super::color::ColorMaterial;
use super::diffuse::DiffuseMaterial;
use super::post_process::PostProcessMaterial;
use super::skybox::SkyboxMaterial;

pub enum Material {
    Color(ColorMaterial),
    Diffuse(DiffuseMaterial),
    Skybox(SkyboxMaterial),
    PostProcess(PostProcessMaterial),
}

impl ApplyMaterial for Material {
    fn apply<'a>(
        &'a mut self,
        encoder: &mut wgpu::RenderBundleEncoder<'a>,
        device: &Device,
        camera: (&Camera, &Transform),
        transform: &Transform,
    ) {
        match self {
            Material::Color(inner) => inner.apply(encoder, device, camera, transform),
            Material::Diffuse(inner) => inner.apply(encoder, device, camera, transform),
            Material::Skybox(inner) => inner.apply(encoder, device, camera, transform),
            Material::PostProcess(inner) => inner.apply(encoder, device, camera, transform),
        }
    }
}

impl Material {
    pub fn color(device: &Device, assets: &Assets) -> Self {
        Self::Color(ColorMaterial::new(device, assets))
    }

    pub fn diffuse(device: &Device, assets: &Assets, texture: &Texture) -> Self {
        Self::Diffuse(DiffuseMaterial::new(device, assets, texture))
    }

    pub fn skybox(device: &Device, assets: &Assets, texture: &Texture) -> Self {
        Self::Skybox(SkyboxMaterial::new(device, assets, texture))
    }

    pub fn post_process(device: &Device, assets: &Assets, texture: &Texture) -> Self {
        Self::PostProcess(PostProcessMaterial::new(device, assets, texture))
    }
}

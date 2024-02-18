use super::super::{Camera, Transform};
use super::color::ColorMaterial;
use super::diffuse::DiffuseMaterial;
use super::post_process::PostProcessMaterial;
use super::skybox::SkyboxMaterial;
use crate::assets::Texture;
use crate::resources::{Assets, Device};
use bevy_ecs::prelude::Component;

#[derive(Component)]
pub enum Material {
    Color(ColorMaterial),
    Diffuse(DiffuseMaterial),
    Skybox(SkyboxMaterial),
    PostProcess(PostProcessMaterial),
}

impl Material {
    pub fn color(device: &Device, assets: &Assets) -> Self {
        Material::Color(ColorMaterial::new(device, assets))
    }

    pub fn diffuse(device: &Device, assets: &Assets, texture: &Texture) -> Self {
        Material::Diffuse(DiffuseMaterial::new(device, assets, texture))
    }

    pub fn skybox(device: &Device, assets: &Assets, texture: &Texture) -> Self {
        Material::Skybox(SkyboxMaterial::new(device, assets, texture))
    }

    pub fn post_process(device: &Device, assets: &Assets, texture: &Texture) -> Self {
        Material::PostProcess(PostProcessMaterial::new(device, assets, texture))
    }

    pub fn apply<'a>(
        &'a mut self,
        device: &Device,
        camera: (&Camera, &Transform),
        transform: &Transform,
        encoder: &mut wgpu::RenderBundleEncoder<'a>,
    ) {
        match self {
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
    }
}

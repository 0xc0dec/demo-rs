use std::path::PathBuf;

use anyhow::*;
use bevy_ecs::prelude::{Commands, Res, Resource};

use crate::device::Device;
use crate::materials::new_shader_module;
use crate::texture::Texture;

fn full_path(relative_path: &str) -> PathBuf {
    std::path::Path::new("./assets").join(relative_path)
}

pub async fn load_binary(file_path: &str) -> Result<Vec<u8>> {
    Ok(std::fs::read(full_path(file_path))?)
}

pub async fn load_string(file_path: &str) -> Result<String> {
    Ok(std::fs::read_to_string(full_path(file_path))?)
}

// TODO Load also shaders, meshes, etc.
#[derive(Resource)]
pub struct Assets {
    pub skybox_tex: Texture,
    pub stone_tex: Texture,
    pub color_shader: wgpu::ShaderModule,
    pub diffuse_shader: wgpu::ShaderModule,
    pub postprocess_shader: wgpu::ShaderModule,
    pub skybox_shader: wgpu::ShaderModule,
}

impl Assets {
    pub fn load(device: Res<Device>, mut commands: Commands) {
        let (
            skybox_tex,
            stone_tex,
            color_shader,
            diffuse_shader,
            postprocess_shader,
            skybox_shader,
        ) = pollster::block_on(async {
            (
                Texture::new_cube_from_file("skybox_bgra.dds", &device)
                    .await
                    .unwrap(),
                Texture::new_2d_from_file("stonewall.jpg", &device)
                    .await
                    .unwrap(),
                new_shader_module(&device, "color.wgsl").await,
                new_shader_module(&device, "diffuse.wgsl").await,
                new_shader_module(&device, "post-process.wgsl").await,
                new_shader_module(&device, "skybox.wgsl").await,
            )
        });

        commands.insert_resource(Self {
            skybox_tex,
            stone_tex,
            color_shader,
            diffuse_shader,
            postprocess_shader,
            skybox_shader,
        })
    }
}

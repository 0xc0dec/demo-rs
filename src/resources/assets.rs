use bevy_ecs::prelude::Resource;
use std::sync::Arc;

use crate::assets::utils::new_shader_module;
use crate::assets::{Mesh, Texture};
use crate::resources::device::Device;

#[derive(Resource)]
pub struct Assets {
    // TODO Public readonly
    pub skybox_tex: Texture,
    pub stone_tex: Texture,
    pub color_shader: wgpu::ShaderModule,
    pub diffuse_shader: wgpu::ShaderModule,
    pub postprocess_shader: wgpu::ShaderModule,
    pub skybox_shader: wgpu::ShaderModule,
    pub box_mesh: Arc<Mesh>,
}

impl Assets {
    pub fn load(device: &Device) -> Self {
        let (
            box_mesh,
            skybox_tex,
            stone_tex,
            color_shader,
            diffuse_shader,
            postprocess_shader,
            skybox_shader,
        ) = pollster::block_on(async {
            (
                Arc::new(Mesh::from_file("cube.obj", device).await),
                Texture::new_cube_from_file("skybox_bgra.dds", device)
                    .await
                    .unwrap(),
                Texture::new_2d_from_file("stonewall.jpg", device)
                    .await
                    .unwrap(),
                new_shader_module(device, "color.wgsl").await,
                new_shader_module(device, "diffuse.wgsl").await,
                new_shader_module(device, "post-process.wgsl").await,
                new_shader_module(device, "skybox.wgsl").await,
            )
        });

        Self {
            box_mesh,
            skybox_tex,
            stone_tex,
            color_shader,
            diffuse_shader,
            postprocess_shader,
            skybox_shader,
        }
    }
}

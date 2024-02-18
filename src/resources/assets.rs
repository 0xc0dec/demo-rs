use bevy_ecs::prelude::{Commands, Res, Resource};

use crate::assets::utils::new_shader_module;
use crate::assets::Texture;
use crate::resources::device::Device;

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
    // TODO Move to the `systems` mod?
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

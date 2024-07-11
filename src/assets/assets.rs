use std::rc::Rc;

use crate::assets::{Mesh, Texture};
use crate::assets::utils::new_shader_module;
use crate::device::Device;

pub struct Assets {
    skybox_tex: Texture,
    stone_tex: Texture,
    color_shader: wgpu::ShaderModule,
    diffuse_shader: wgpu::ShaderModule,
    postprocess_shader: wgpu::ShaderModule,
    skybox_shader: wgpu::ShaderModule,
    box_mesh: Rc<Mesh>,
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
                Rc::new(Mesh::from_file("cube.obj", device).await),
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

    pub fn skybox_texture(&self) -> &Texture {
        &self.skybox_tex
    }

    pub fn stone_texture(&self) -> &Texture {
        &self.stone_tex
    }

    pub fn color_shader(&self) -> &wgpu::ShaderModule {
        &self.color_shader
    }

    pub fn diffuse_shader(&self) -> &wgpu::ShaderModule {
        &self.diffuse_shader
    }

    pub fn postprocess_shader(&self) -> &wgpu::ShaderModule {
        &self.postprocess_shader
    }

    pub fn skybox_shader(&self) -> &wgpu::ShaderModule {
        &self.skybox_shader
    }

    pub fn box_mesh(&self) -> Rc<Mesh> {
        Rc::clone(&self.box_mesh)
    }
}

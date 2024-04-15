use super::super::device::Device;
use super::Texture;
use super::utils::new_shader_module;

pub struct Assets {
    pub skybox_tex: Texture,
    pub stone_tex: Texture,
    pub color_shader: wgpu::ShaderModule,
    pub diffuse_shader: wgpu::ShaderModule,
    pub postprocess_shader: wgpu::ShaderModule,
    pub skybox_shader: wgpu::ShaderModule,
}

impl Assets {
    pub fn load(device: &Device) -> Self {
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

        Self {
            skybox_tex,
            stone_tex,
            color_shader,
            diffuse_shader,
            postprocess_shader,
            skybox_shader,
        }
    }
}

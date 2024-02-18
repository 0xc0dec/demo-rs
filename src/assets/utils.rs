use anyhow::*;
use std::path::PathBuf;

fn full_path(relative_path: &str) -> PathBuf {
    std::path::Path::new("./assets").join(relative_path)
}

pub async fn load_binary(file_path: &str) -> Result<Vec<u8>> {
    Ok(std::fs::read(full_path(file_path))?)
}

pub async fn load_string(file_path: &str) -> Result<String> {
    Ok(std::fs::read_to_string(full_path(file_path))?)
}

pub async fn new_shader_module(device: &wgpu::Device, src_file_path: &str) -> wgpu::ShaderModule {
    let src = load_string(src_file_path).await.unwrap();

    device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: None,
        source: wgpu::ShaderSource::Wgsl(src.into()),
    })
}

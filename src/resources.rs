use anyhow::*;
use crate::texture::Texture;

async fn load_binary(res_file_path: &str) -> Result<Vec<u8>> {
    let path = std::path::Path::new("./res").join(res_file_path);
    let data = std::fs::read(path)?;
    Ok(data)
}

pub async fn load_texture(
    file_name: &str,
    device: &wgpu::Device,
    queue: &wgpu::Queue,
) -> Result<Texture> {
    let data = load_binary(file_name).await?;
    Texture::from_bytes(device, queue, &data)
}

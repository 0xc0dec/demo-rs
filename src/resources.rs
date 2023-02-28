use anyhow::*;

pub async fn load_binary(res_file_path: &str) -> Result<Vec<u8>> {
    let path = std::path::Path::new("./res").join(res_file_path);
    let data = std::fs::read(path)?;
    Ok(data)
}

pub async fn load_string(file_name: &str) -> Result<String> {
    let path = std::path::Path::new("./res").join(file_name);
    Ok(std::fs::read_to_string(path)?)
}
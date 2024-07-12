use std::path::PathBuf;

use anyhow::*;

fn full_path(relative_path: &str) -> PathBuf {
    std::path::Path::new("./assets").join(relative_path)
}

pub async fn load_binary_asset(file_path: &str) -> Result<Vec<u8>> {
    Ok(std::fs::read(full_path(file_path))?)
}

pub async fn load_string_asset(file_path: &str) -> Result<String> {
    Ok(std::fs::read_to_string(full_path(file_path))?)
}

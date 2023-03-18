use anyhow::*;
use std::path::PathBuf;

fn full_res_path(relative_path: &str) -> PathBuf {
    std::path::Path::new("./res").join(relative_path)
}

pub async fn load_binary(res_file_path: &str) -> Result<Vec<u8>> {
    let path = full_res_path(res_file_path);
    Ok(std::fs::read(path)?)
}

pub async fn load_string(res_file_path: &str) -> Result<String> {
    let path = full_res_path(res_file_path);
    Ok(std::fs::read_to_string(path)?)
}

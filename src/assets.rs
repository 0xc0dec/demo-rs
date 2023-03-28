use crate::device::Device;
use crate::mesh::Model;
use crate::texture::Texture;
use anyhow::*;
use std::collections::HashMap;
use std::path::PathBuf;
use std::rc::Rc;

fn full_path(relative_path: &str) -> PathBuf {
    std::path::Path::new("./assets").join(relative_path)
}

pub async fn load_binary(file_path: &str) -> Result<Vec<u8>> {
    Ok(std::fs::read(full_path(file_path))?)
}

pub async fn load_string(file_path: &str) -> Result<String> {
    Ok(std::fs::read_to_string(full_path(file_path))?)
}

pub struct Assets {
    models: HashMap<String, Rc<Model>>,
    textures: HashMap<String, Rc<Texture>>,
}

// TODO Other types of assets
impl Assets {
    pub fn new() -> Self {
        Self {
            models: HashMap::new(),
            textures: HashMap::new(),
        }
    }

    pub async fn model(&mut self, file_name: &str, device: &Device) -> Rc<Model> {
        if !self.models.contains_key(file_name) {
            println!("Model {file_name} not found in cache, loading...");
            let model = Model::from_file(file_name, device)
                .await
                .expect(format!("Unable to load model {file_name}").as_str());
            self.models.insert(file_name.to_owned(), Rc::new(model));
            println!("Loaded model {file_name}");
        }
        Rc::clone(self.models.get(file_name).unwrap())
    }

    pub async fn texture_2d(&mut self, file_name: &str, device: &Device) -> Rc<Texture> {
        if !self.textures.contains_key(file_name) {
            println!("2d texture {file_name} not found in cache, loading...");
            let texture = Texture::new_2d_from_file(file_name, device)
                .await
                .expect(format!("Unable to load 2d texture {file_name}").as_str());
            self.textures.insert(file_name.to_owned(), Rc::new(texture));
            println!("Loaded 2d texture {file_name}");
        }
        Rc::clone(self.textures.get(file_name).unwrap())
    }
}

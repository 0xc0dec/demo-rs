use std::collections::HashMap;
use anyhow::*;
use std::path::PathBuf;
use std::rc::Rc;
use crate::device::Device;
use crate::model::Model;

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

pub struct Resources {
    models: HashMap<String, Rc<Model>>,
}

impl Resources {
    pub fn new() -> Self {
        Self {
            models: HashMap::new()
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
}
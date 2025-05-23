use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub enum ColliderShapeCfg {
    Cube,
}

#[derive(Deserialize, Debug)]
pub enum ComponentCfg {
    Mesh { path: String },
    Material { name: String },
    PlayerTarget,
}

#[derive(Deserialize, Debug)]
pub enum MaterialCfg {
    Color {
        name: String,
        color: [f32; 3],
        wireframe: Option<bool>,
    },
    Textured {
        name: String,
        texture: String,
    },
}

#[derive(Deserialize, Debug)]
pub struct BodyCfg {
    pub shape: ColliderShapeCfg,
    pub movable: Option<bool>,
}

#[derive(Deserialize, Debug)]
pub struct NodeCfg {
    pub render_order: i32,
    pub render_tags: u32,
    pub pos: Option<[f32; 3]>,
    pub scale: Option<[f32; 3]>,
    pub body: Option<BodyCfg>,
    pub components: Vec<ComponentCfg>,
}

#[derive(Deserialize, Debug)]
pub struct SceneCfg {
    pub materials: Vec<MaterialCfg>,
    pub nodes: HashMap<String, NodeCfg>,
}

impl SceneCfg {
    pub fn from_yaml(yaml: &str) -> Self {
        serde_yaml::from_str::<SceneCfg>(yaml).unwrap()
    }
}

mod tests {
    use super::*;

    #[test]
    fn smoke() {
        let def_file_content = String::from_utf8_lossy(include_bytes!("../../assets/scene.yml"));
        let config = serde_yaml::from_str::<SceneCfg>(&def_file_content).unwrap();
        println!("{:?}", config);
    }
}

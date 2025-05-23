use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub enum ColliderShapeDef {
    Cube,
}

#[derive(Deserialize, Debug)]
pub enum ComponentDef {
    Mesh {
        path: String,
    },
    Material {
        name: String,
    },
    Body {
        shape: ColliderShapeDef,
        movable: Option<bool>,
    },
}

#[derive(Deserialize, Debug)]
pub enum MaterialDef {
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
pub struct NodeDef {
    pub render_order: i32,
    pub render_tags: u32,
    pub pos: Option<[f32; 3]>,
    pub scale: Option<[f32; 3]>,
    pub components: Vec<ComponentDef>,
}

#[derive(Deserialize, Debug)]
pub struct SceneDef {
    pub materials: Vec<MaterialDef>,
    pub nodes: HashMap<String, NodeDef>,
}

impl SceneDef {
    pub fn from_yaml(yaml: &str) -> Self {
        serde_yaml::from_str::<SceneDef>(yaml).unwrap()
    }
}

mod tests {
    use super::*;

    #[test]
    fn smoke() {
        let def_file_content = String::from_utf8_lossy(include_bytes!("../../assets/scene.yml"));
        let config = serde_yaml::from_str::<SceneDef>(&def_file_content).unwrap();
        println!("{:?}", config);
    }
}

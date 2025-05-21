use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
enum ComponentDef {
    Transform { pos: [f32; 3], scale: [f32; 3] },
    Body { weight: f32 },
    Mesh { name: String },
    Material { name: String },
}

#[derive(Deserialize, Debug)]
struct MaterialDef {
    shader: String,
    wireframe: Option<bool>,
}

#[derive(Deserialize, Debug)]
pub struct SceneDef {
    materials: HashMap<String, MaterialDef>,
    nodes: HashMap<String, Vec<ComponentDef>>,
}

mod tests {
    use super::*;

    #[test]
    fn smoke() {
        let config = "
          materials:
            textured:
              shader: textured.wsgl
            color:
              shader: color.wsgl
              wireframe: true
          nodes:
            one:
            - !Transform
              pos: [1, 2, 3]
              scale: [1, 1, 1]
            - !Body
              weight: 12.3
            - !Mesh
              name: cube.obj
            - !Material
              name: textured
        ";
        let config = serde_yaml::from_str::<SceneDef>(config).unwrap();
        println!("{:?}", config);
    }
}

use super::{ColorMaterial, PostProcessMaterial, SkyboxMaterial, TexturedMaterial};

pub enum Material {
    Color(ColorMaterial),
    Skybox(SkyboxMaterial),
    Textured(TexturedMaterial),
    PostProcess(PostProcessMaterial),
}

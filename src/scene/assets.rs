use super::materials::{
    ColorMaterial, Material, PostProcessMaterial, SkyboxMaterial, TexturedMaterial,
};
use crate::file;
use crate::math::Vec3;
use crate::render::Mesh;
use crate::render::Renderer;
use crate::render::Texture;
use futures_lite::future;
use slotmap::{DefaultKey, SlotMap};

pub type MeshHandle = DefaultKey;
pub type MaterialHandle = DefaultKey;
pub type ShaderHandle = DefaultKey;
pub type TextureHandle = DefaultKey;

pub struct Assets {
    textures: SlotMap<TextureHandle, Texture>,

    pub color_shader: ShaderHandle,
    pub textured_shader: ShaderHandle,
    pub skybox_shader: ShaderHandle,
    pub postprocess_shader: ShaderHandle,
    shaders: SlotMap<ShaderHandle, wgpu::ShaderModule>,

    meshes: SlotMap<MeshHandle, Mesh>,
    materials: SlotMap<MaterialHandle, Material>,
}

// TODO Remove hardcoded assets, make scene add them.
// TODO Add lookup by name/path. Should return handles for further faster lookup.
impl Assets {
    pub fn load(rr: &Renderer) -> Self {
        let (color_shader, textured_shader, postprocess_shader, skybox_shader) =
            future::block_on(async {
                (
                    new_shader_module(rr, "color.wgsl").await,
                    new_shader_module(rr, "textured.wgsl").await,
                    new_shader_module(rr, "post-process.wgsl").await,
                    new_shader_module(rr, "skybox.wgsl").await,
                )
            });

        let mut shaders = SlotMap::new();
        let color_shader = shaders.insert(color_shader);
        let textured_shader = shaders.insert(textured_shader);
        let postprocess_shader = shaders.insert(postprocess_shader);
        let skybox_shader = shaders.insert(skybox_shader);

        let mut textures = SlotMap::new();

        Self {
            textures,
            shaders,
            color_shader,
            textured_shader,
            postprocess_shader,
            skybox_shader,
            meshes: SlotMap::new(),
            materials: SlotMap::new(),
        }
    }

    pub fn mesh(&self, handle: MeshHandle) -> &Mesh {
        self.meshes.get(handle).unwrap()
    }

    pub fn shader(&self, handle: ShaderHandle) -> &wgpu::ShaderModule {
        self.shaders.get(handle).unwrap()
    }

    pub fn add_mesh_from_file(&mut self, rr: &Renderer, path: &str) -> MeshHandle {
        self.meshes
            .insert(future::block_on(Mesh::from_file(rr, path)))
    }

    pub fn add_mesh(&mut self, mesh: Mesh) -> MeshHandle {
        self.meshes.insert(mesh)
    }

    pub fn add_2d_texture_from_file(&mut self, rr: &Renderer, path: &str) -> TextureHandle {
        let tex = future::block_on(Texture::new_2d_from_file(path, rr));
        self.textures.insert(tex.unwrap())
    }

    pub fn add_cube_texture_from_file(&mut self, rr: &Renderer, path: &str) -> TextureHandle {
        let tex = future::block_on(Texture::new_cube_from_file(path, rr));
        self.textures.insert(tex.unwrap())
    }

    pub fn add_color_material(
        &mut self,
        rr: &Renderer,
        color: Vec3,
        wireframe: bool,
    ) -> MaterialHandle {
        self.materials.insert(Material::Color(ColorMaterial::new(
            rr, self, color, wireframe,
        )))
    }

    pub fn add_skybox_material(&mut self, rr: &Renderer, texture: TextureHandle) -> MaterialHandle {
        self.materials.insert(Material::Skybox(SkyboxMaterial::new(
            rr,
            self,
            &self.textures[texture],
        )))
    }

    pub fn add_textured_material(
        &mut self,
        rr: &Renderer,
        texture: TextureHandle,
    ) -> MaterialHandle {
        self.materials
            .insert(Material::Textured(TexturedMaterial::new(
                rr,
                self,
                &self.textures[texture],
            )))
    }

    pub fn add_postprocess_material(
        &mut self,
        rr: &Renderer,
        src_texture: &Texture,
    ) -> MaterialHandle {
        self.materials
            .insert(Material::PostProcess(PostProcessMaterial::new(
                rr,
                self,
                src_texture,
            )))
    }

    pub fn remove_material(&mut self, handle: MaterialHandle) {
        self.materials.remove(handle);
    }

    pub fn material(&self, handle: MaterialHandle) -> &Material {
        &self.materials[handle]
    }
}

async fn new_shader_module(device: &wgpu::Device, src_file_path: &str) -> wgpu::ShaderModule {
    let src = file::read_string_asset(src_file_path).await.unwrap();
    device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: None,
        source: wgpu::ShaderSource::Wgsl(src.into()),
    })
}

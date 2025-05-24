use super::materials::{ColorMaterial, Material};
use crate::file;
use crate::math::Vec3;
use crate::render::Mesh;
use crate::render::Renderer;
use crate::render::Texture;
use futures_lite::future;
use slotmap::{DefaultKey, SlotMap};
use std::collections::HashMap;

pub type MeshHandle = DefaultKey;
pub type MaterialHandle = DefaultKey;
pub type ShaderHandle = DefaultKey;
pub type TextureHandle = DefaultKey;

pub struct Assets {
    textures: SlotMap<TextureHandle, Texture>,
    texture_handles: HashMap<String, TextureHandle>,
    shaders: SlotMap<ShaderHandle, wgpu::ShaderModule>,
    shader_handles: HashMap<String, ShaderHandle>,
    meshes: SlotMap<MeshHandle, Mesh>,
    materials: SlotMap<MaterialHandle, Material>,

    pub color_shader: ShaderHandle,
}

// TODO Remove hardcoded assets, make scene add them.
// TODO Add lookup by name/path. Should return handles for further faster lookup.
impl Assets {
    pub fn load(rr: &Renderer) -> Self {
        let (color_shader,) =
            future::block_on(async { (new_shader_module(rr, "color.wgsl").await,) });

        let mut shaders = SlotMap::new();
        let color_shader = shaders.insert(color_shader);

        Self {
            textures: SlotMap::new(),
            texture_handles: HashMap::new(),
            meshes: SlotMap::new(),
            materials: SlotMap::new(),
            shaders,
            shader_handles: HashMap::new(),
            color_shader,
        }
    }

    pub fn shader(&self, handle: ShaderHandle) -> &wgpu::ShaderModule {
        self.shaders.get(handle).unwrap()
    }

    pub fn add_shader_from_file(&mut self, rr: &Renderer, path: &str) -> ShaderHandle {
        *self
            .shader_handles
            .entry(path.to_string())
            .or_insert_with(|| {
                self.shaders
                    .insert(future::block_on(new_shader_module(rr, path)))
            })
    }

    pub fn mesh(&self, handle: MeshHandle) -> &Mesh {
        self.meshes.get(handle).unwrap()
    }

    pub fn add_mesh(&mut self, mesh: Mesh) -> MeshHandle {
        self.meshes.insert(mesh)
    }

    pub fn add_mesh_from_file(&mut self, rr: &Renderer, path: &str) -> MeshHandle {
        self.meshes
            .insert(future::block_on(Mesh::from_file(rr, path)))
    }

    pub fn texture(&self, handle: TextureHandle) -> &Texture {
        self.textures.get(handle).unwrap()
    }

    pub fn add_2d_texture_from_file(&mut self, rr: &Renderer, path: &str) -> TextureHandle {
        self.add_texture(path, || {
            future::block_on(Texture::new_2d_from_file(path, rr)).unwrap()
        })
    }

    pub fn add_cube_texture_from_file(&mut self, rr: &Renderer, path: &str) -> TextureHandle {
        self.add_texture(path, || {
            future::block_on(Texture::new_cube_from_file(path, rr)).unwrap()
        })
    }

    fn add_texture(&mut self, key: &str, new_texture: impl FnOnce() -> Texture) -> TextureHandle {
        *self
            .texture_handles
            .entry(key.to_string())
            .or_insert_with(|| self.textures.insert(new_texture()))
    }

    pub fn material(&self, handle: MaterialHandle) -> &Material {
        &self.materials[handle]
    }

    pub fn add_material(&mut self, material: Material) -> MaterialHandle {
        self.materials.insert(material)
    }

    pub fn remove_material(&mut self, handle: MaterialHandle) {
        self.materials.remove(handle);
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
}

async fn new_shader_module(device: &wgpu::Device, src_file_path: &str) -> wgpu::ShaderModule {
    let src = file::read_string_asset(src_file_path).await.unwrap();
    device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: None,
        source: wgpu::ShaderSource::Wgsl(src.into()),
    })
}

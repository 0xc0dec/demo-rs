use futures_lite::future;
use slotmap::{DefaultKey, SlotMap};

use crate::file;
use crate::materials::{
    ColorMaterial, Material, PostProcessMaterial, SkyboxMaterial, TexturedMaterial,
};
use crate::mesh::Mesh;
use crate::render::Renderer;
use crate::render::Texture;

pub type MeshHandle = DefaultKey;
pub type MaterialHandle = DefaultKey;
pub type ShaderHandle = DefaultKey;
pub type TextureHandle = DefaultKey;

pub struct Assets {
    pub bricks_texture: TextureHandle,
    pub crate_texture: TextureHandle,
    pub skybox_texture: TextureHandle,
    textures: SlotMap<TextureHandle, Texture>,

    pub color_shader: ShaderHandle,
    pub textured_shader: ShaderHandle,
    pub skybox_shader: ShaderHandle,
    pub postprocess_shader: ShaderHandle,
    shaders: SlotMap<ShaderHandle, wgpu::ShaderModule>,

    pub box_mesh: MeshHandle,
    pub quad_mesh: MeshHandle,
    meshes: SlotMap<MeshHandle, Mesh>,

    materials: SlotMap<MaterialHandle, Material>,
}

impl Assets {
    pub fn load(rr: &Renderer) -> Self {
        let (
            box_mesh,
            skybox_tex,
            bricks_tex,
            crate_tex,
            color_shader,
            textured_shader,
            postprocess_shader,
            skybox_shader,
        ) = future::block_on(async {
            (
                Mesh::from_file(rr, "cube.obj").await,
                Texture::new_cube_from_file("skybox_bgra.dds", rr)
                    .await
                    .unwrap(),
                Texture::new_2d_from_file("bricks.png", rr).await.unwrap(),
                Texture::new_2d_from_file("crate.png", rr).await.unwrap(),
                new_shader_module(rr, "color.wgsl").await,
                new_shader_module(rr, "textured.wgsl").await,
                new_shader_module(rr, "post-process.wgsl").await,
                new_shader_module(rr, "skybox.wgsl").await,
            )
        });

        let mut meshes = SlotMap::new();
        let box_mesh = meshes.insert(box_mesh);
        let quad_mesh = meshes.insert(Mesh::new_quad(rr));

        let mut shaders = SlotMap::new();
        let color_shader = shaders.insert(color_shader);
        let textured_shader = shaders.insert(textured_shader);
        let postprocess_shader = shaders.insert(postprocess_shader);
        let skybox_shader = shaders.insert(skybox_shader);

        let mut textures = SlotMap::new();
        let bricks_texture = textures.insert(bricks_tex);
        let skybox_texture = textures.insert(skybox_tex);
        let crate_texture = textures.insert(crate_tex);

        Self {
            textures,
            bricks_texture,
            crate_texture,
            skybox_texture,
            shaders,
            color_shader,
            textured_shader,
            postprocess_shader,
            skybox_shader,
            meshes,
            box_mesh,
            quad_mesh,
            materials: SlotMap::new(),
        }
    }

    pub fn mesh(&self, handle: MeshHandle) -> &Mesh {
        self.meshes.get(handle).unwrap()
    }

    pub fn shader(&self, handle: ShaderHandle) -> &wgpu::ShaderModule {
        self.shaders.get(handle).unwrap()
    }

    pub fn add_color_material(&mut self, rr: &Renderer) -> MaterialHandle {
        self.materials
            .insert(Material::Color(ColorMaterial::new(rr, self)))
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

    pub fn material_mut(&mut self, handle: MaterialHandle) -> &mut Material {
        &mut self.materials[handle]
    }
}

async fn new_shader_module(device: &wgpu::Device, src_file_path: &str) -> wgpu::ShaderModule {
    let src = file::read_string_asset(src_file_path).await.unwrap();
    device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: None,
        source: wgpu::ShaderSource::Wgsl(src.into()),
    })
}

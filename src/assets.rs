use slotmap::{DefaultKey, SlotMap};

use crate::fs::load_string_asset;
use crate::graphics::Graphics;
use crate::mesh::Mesh;
use crate::texture::Texture;

pub type MeshHandle = DefaultKey;
// Materials are not stored in Assets because they're scene-specific, however I don't know a better place where
// to put this type.
pub type MaterialHandle = DefaultKey;
pub type ShaderHandle = DefaultKey;
pub type TextureHandle = DefaultKey;

pub struct Assets {
    pub stone_texture_handle: TextureHandle,
    pub skybox_texture_handle: TextureHandle,
    textures: SlotMap<TextureHandle, Texture>,

    pub color_shader_handle: ShaderHandle,
    pub textured_shader_handle: ShaderHandle,
    pub skybox_shader_handle: ShaderHandle,
    pub postprocess_shader_handle: ShaderHandle,
    shaders: SlotMap<ShaderHandle, wgpu::ShaderModule>,

    pub box_mesh_handle: MeshHandle,
    pub quad_mesh_handle: MeshHandle,
    meshes: SlotMap<MeshHandle, Mesh>,
}

impl Assets {
    pub fn load(gfx: &Graphics) -> Self {
        let (
            box_mesh,
            skybox_tex,
            stone_tex,
            color_shader,
            textured_shader,
            postprocess_shader,
            skybox_shader,
        ) = pollster::block_on(async {
            (
                Mesh::from_file("cube.obj", gfx).await,
                Texture::new_cube_from_file("skybox_bgra.dds", gfx)
                    .await
                    .unwrap(),
                Texture::new_2d_from_file("stonewall.jpg", gfx)
                    .await
                    .unwrap(),
                new_shader_module(gfx, "color.wgsl").await,
                new_shader_module(gfx, "textured.wgsl").await,
                new_shader_module(gfx, "post-process.wgsl").await,
                new_shader_module(gfx, "skybox.wgsl").await,
            )
        });

        let mut meshes = SlotMap::new();
        let box_mesh_handle = meshes.insert(box_mesh);
        let quad_mesh_handle = meshes.insert(Mesh::quad(gfx));

        let mut shaders = SlotMap::new();
        let color_shader_handle = shaders.insert(color_shader);
        let textured_shader_handle = shaders.insert(textured_shader);
        let postprocess_shader_handle = shaders.insert(postprocess_shader);
        let skybox_shader_handle = shaders.insert(skybox_shader);

        let mut textures = SlotMap::new();
        let stone_texture_handle = textures.insert(stone_tex);
        let skybox_texture_handle = textures.insert(skybox_tex);

        Self {
            textures,
            stone_texture_handle,
            skybox_texture_handle,
            shaders,
            color_shader_handle,
            textured_shader_handle,
            postprocess_shader_handle,
            skybox_shader_handle,
            meshes,
            box_mesh_handle,
            quad_mesh_handle,
        }
    }

    pub fn texture(&self, handle: TextureHandle) -> &Texture {
        self.textures.get(handle).unwrap()
    }

    pub fn mesh(&self, handle: MeshHandle) -> &Mesh {
        self.meshes.get(handle).unwrap()
    }

    pub fn shader(&self, handle: ShaderHandle) -> &wgpu::ShaderModule {
        self.shaders.get(handle).unwrap()
    }
}

async fn new_shader_module(device: &wgpu::Device, src_file_path: &str) -> wgpu::ShaderModule {
    let src = load_string_asset(src_file_path).await.unwrap();
    device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: None,
        source: wgpu::ShaderSource::Wgsl(src.into()),
    })
}

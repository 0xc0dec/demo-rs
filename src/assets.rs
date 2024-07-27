use slotmap::{DefaultKey, SlotMap};

use crate::fs::load_string_asset;
use crate::graphics::Graphics;
use crate::mesh::Mesh;
use crate::texture::Texture;

pub type MeshId = DefaultKey;
pub type ShaderId = DefaultKey;
pub type TextureId = DefaultKey;

pub struct Assets {
    pub stone_texture_id: TextureId,
    pub skybox_texture_id: TextureId,
    textures: SlotMap<TextureId, Texture>,

    pub color_shader_id: ShaderId,
    pub textured_shader_id: ShaderId,
    pub skybox_shader_id: ShaderId,
    pub postprocess_shader_id: ShaderId,
    shaders: SlotMap<ShaderId, wgpu::ShaderModule>,

    pub box_mesh_id: MeshId,
    pub quad_mesh_id: MeshId,
    meshes: SlotMap<MeshId, Mesh>,
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
        let box_mesh_id = meshes.insert(box_mesh);
        let quad_mesh_id = meshes.insert(Mesh::quad(gfx));

        let mut shaders = SlotMap::new();
        let color_shader_id = shaders.insert(color_shader);
        let textured_shader_id = shaders.insert(textured_shader);
        let postprocess_shader_id = shaders.insert(postprocess_shader);
        let skybox_shader_id = shaders.insert(skybox_shader);

        let mut textures = SlotMap::new();
        let stone_texture_id = textures.insert(stone_tex);
        let skybox_texture_id = textures.insert(skybox_tex);

        Self {
            textures,
            stone_texture_id,
            skybox_texture_id,
            shaders,
            color_shader_id,
            textured_shader_id,
            postprocess_shader_id,
            skybox_shader_id,
            meshes,
            box_mesh_id,
            quad_mesh_id,
        }
    }

    pub fn texture(&self, id: TextureId) -> &Texture {
        self.textures.get(id).unwrap()
    }

    pub fn mesh(&self, id: MeshId) -> &Mesh {
        self.meshes.get(id).unwrap()
    }

    pub fn shader(&self, id: ShaderId) -> &wgpu::ShaderModule {
        self.shaders.get(id).unwrap()
    }
}

async fn new_shader_module(device: &wgpu::Device, src_file_path: &str) -> wgpu::ShaderModule {
    let src = load_string_asset(src_file_path).await.unwrap();
    device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: None,
        source: wgpu::ShaderSource::Wgsl(src.into()),
    })
}

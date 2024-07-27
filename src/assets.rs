use slotmap::{DefaultKey, SlotMap};

use crate::fs::load_string_asset;
use crate::graphics::Graphics;
use crate::mesh::Mesh;
use crate::texture::Texture;

pub type MeshId = DefaultKey;

// TODO Store groups of assets in hash maps
pub struct Assets {
    skybox_tex: Texture,
    stone_tex: Texture,
    color_shader: wgpu::ShaderModule,
    textured_shader: wgpu::ShaderModule,
    postprocess_shader: wgpu::ShaderModule,
    skybox_shader: wgpu::ShaderModule,
    meshes: SlotMap<MeshId, Mesh>,
    box_mesh_id: MeshId,
    quad_mesh_id: MeshId,
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

        Self {
            skybox_tex,
            stone_tex,
            color_shader,
            textured_shader,
            postprocess_shader,
            skybox_shader,
            meshes,
            box_mesh_id,
            quad_mesh_id,
        }
    }

    pub fn skybox_texture(&self) -> &Texture {
        &self.skybox_tex
    }

    pub fn stone_texture(&self) -> &Texture {
        &self.stone_tex
    }

    pub fn color_shader(&self) -> &wgpu::ShaderModule {
        &self.color_shader
    }

    pub fn textured_shader(&self) -> &wgpu::ShaderModule {
        &self.textured_shader
    }

    pub fn postprocess_shader(&self) -> &wgpu::ShaderModule {
        &self.postprocess_shader
    }

    pub fn skybox_shader(&self) -> &wgpu::ShaderModule {
        &self.skybox_shader
    }

    pub fn box_mesh_id(&self) -> MeshId {
        self.box_mesh_id
    }

    pub fn quad_mesh_id(&self) -> MeshId {
        self.quad_mesh_id
    }

    pub fn mesh(&self, id: MeshId) -> &Mesh {
        self.meshes.get(id).unwrap()
    }
}

async fn new_shader_module(device: &wgpu::Device, src_file_path: &str) -> wgpu::ShaderModule {
    let src = load_string_asset(src_file_path).await.unwrap();

    device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: None,
        source: wgpu::ShaderSource::Wgsl(src.into()),
    })
}

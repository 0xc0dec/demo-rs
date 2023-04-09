use anyhow::*;
use std::path::PathBuf;
use bevy_ecs::prelude::{Commands, Res, Resource};
use crate::device::Device;
use crate::texture::Texture;

fn full_path(relative_path: &str) -> PathBuf {
    std::path::Path::new("./assets").join(relative_path)
}

pub async fn load_binary(file_path: &str) -> Result<Vec<u8>> {
    Ok(std::fs::read(full_path(file_path))?)
}

pub async fn load_string(file_path: &str) -> Result<String> {
    Ok(std::fs::read_to_string(full_path(file_path))?)
}

// TODO Load also shaders, meshes, etc.
#[derive(Resource)]
pub struct Assets {
    pub skybox_tex: Texture,
    pub stone_tex: Texture,
}

impl Assets {
    pub fn load(device: Res<Device>, mut commands: Commands) {
        let (skybox_tex, stone_tex) = pollster::block_on(async {
            let skybox_tex = Texture::new_cube_from_file("skybox_bgra.dds", &device)
                .await
                .unwrap();
            let stone_tex = Texture::new_2d_from_file("stonewall.jpg", &device)
                .await
                .unwrap();
            (skybox_tex, stone_tex)
        });

        commands.insert_resource(Self {
            skybox_tex,
            stone_tex
        })
    }
}
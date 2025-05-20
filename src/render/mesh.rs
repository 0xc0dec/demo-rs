use std::io::{BufReader, Cursor};
use wgpu::util::DeviceExt;

use crate::file;

use super::vertex::PosTexCoordNormalVertex;

struct MeshPart {
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    num_indices: u32,
}

impl MeshPart {
    fn from_buffers(
        device: &wgpu::Device,
        vertices: &[PosTexCoordNormalVertex],
        indices: &[u32],
    ) -> Self {
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(indices),
            usage: wgpu::BufferUsages::INDEX,
        });

        Self {
            vertex_buffer,
            index_buffer,
            num_indices: indices.len() as u32,
        }
    }

    // TODO Use different vertex description and remove unused attributes
    fn new_quad(device: &wgpu::Device) -> MeshPart {
        let vertices = vec![
            // Bottom left
            PosTexCoordNormalVertex {
                position: [-1.0, -1.0, 0.0],
                tex_coords: [0.0, 0.0],
                normal: [0.0, 0.0, 0.0], // unused
            },
            // Top left
            PosTexCoordNormalVertex {
                position: [-1.0, 1.0, 0.0],
                tex_coords: [0.0, 1.0],
                normal: [0.0, 0.0, 0.0], // unused
            },
            // Top right
            PosTexCoordNormalVertex {
                position: [1.0, 1.0, 0.0],
                tex_coords: [1.0, 1.0],
                normal: [0.0, 0.0, 0.0], // unused
            },
            // Bottom right
            PosTexCoordNormalVertex {
                position: [1.0, -1.0, 0.0],
                tex_coords: [1.0, 0.0],
                normal: [0.0, 0.0, 0.0], // unused
            },
        ];

        let indices: Vec<u32> = vec![0, 1, 2, 0, 2, 3];

        Self::from_buffers(device, &vertices, &indices)
    }
}

pub struct Mesh {
    parts: Vec<MeshPart>,
}

impl Mesh {
    pub fn new_quad(device: &wgpu::Device) -> Self {
        Self {
            parts: vec![MeshPart::new_quad(device)],
        }
    }

    // TODO Extract the file reading, this struct is only for rendering.
    pub async fn from_file(device: &wgpu::Device, file_path: &str) -> Mesh {
        let text = file::read_string_asset(file_path).await.unwrap();
        let cursor = futures_lite::io::Cursor::new(text);
        let mut reader = futures_lite::io::BufReader::new(cursor);

        let (meshes, _) = tobj::futures::load_obj_buf(
            &mut reader,
            &tobj::LoadOptions {
                triangulate: true,
                single_index: true,
                ..Default::default()
            },
            |p| async move {
                let mat_text = file::read_string_asset(p.to_str().unwrap()).await.unwrap();
                tobj::load_mtl_buf(&mut BufReader::new(Cursor::new(mat_text)))
            },
        )
        .await
        .unwrap();

        let parts = meshes
            .into_iter()
            .map(|m| {
                let vertices = (0..m.mesh.positions.len() / 3)
                    .map(|i| PosTexCoordNormalVertex {
                        position: [
                            m.mesh.positions[i * 3],
                            m.mesh.positions[i * 3 + 1],
                            m.mesh.positions[i * 3 + 2],
                        ],
                        tex_coords: [m.mesh.texcoords[i * 2], m.mesh.texcoords[i * 2 + 1]],
                        normal: [
                            m.mesh.normals[i * 3],
                            m.mesh.normals[i * 3 + 1],
                            m.mesh.normals[i * 3 + 2],
                        ],
                    })
                    .collect::<Vec<_>>();

                MeshPart::from_buffers(device, &vertices, &m.mesh.indices)
            })
            .collect::<Vec<_>>();

        Mesh { parts }
    }

    pub fn draw<'a>(&'a self, encoder: &mut wgpu::RenderBundleEncoder<'a>) {
        for part in &self.parts {
            encoder.set_vertex_buffer(0, part.vertex_buffer.slice(..));
            encoder.set_index_buffer(part.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
            encoder.draw_indexed(0..part.num_indices, 0, 0..1);
        }
    }
}

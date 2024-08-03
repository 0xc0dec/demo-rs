use anyhow::*;
use image::GenericImageView;
use wgpu::util::{DeviceExt, TextureDataOrder};

use crate::file;
use crate::graphics::Graphics;

pub type TextureSize = (u32, u32);

pub struct Texture {
    view: wgpu::TextureView,
    sampler: wgpu::Sampler,
    format: wgpu::TextureFormat,
}

impl Texture {
    pub fn new_depth(
        gfx: &wgpu::Device,
        format: wgpu::TextureFormat,
        size: TextureSize,
    ) -> Self {
        let size = wgpu::Extent3d {
            width: size.0,
            height: size.1,
            depth_or_array_layers: 1,
        };

        let texture = gfx.create_texture(&wgpu::TextureDescriptor {
            label: None,
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        });

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = gfx.create_sampler(&new_sampler_descriptor(wgpu::FilterMode::Nearest,
            wgpu::FilterMode::Nearest,
            Some(wgpu::CompareFunction::LessEqual),
        ));

        Self {
            view,
            sampler,
            format,
        }
    }

    pub fn new_render_attachment(
        gfx: &wgpu::Device,
        format: wgpu::TextureFormat,
        size: TextureSize,
    ) -> Self {
        let size = wgpu::Extent3d {
            width: size.0,
            height: size.1,
            depth_or_array_layers: 1,
        };

        let texture = gfx.create_texture(&wgpu::TextureDescriptor {
            label: None,
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        });

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = gfx.create_sampler(&new_sampler_descriptor(
            wgpu::FilterMode::Nearest,
            wgpu::FilterMode::Nearest,
            None,
        ));

        Self {
            view,
            sampler,
            format,
        }
    }

    pub async fn new_2d_from_file(file_name: &str, gfx: &Graphics<'_>) -> Result<Self> {
        let data = file::read_binary_asset(file_name).await?;
        Self::new_2d_from_mem(gfx, &data)
    }

    pub async fn new_cube_from_file(file_name: &str, gfx: &Graphics<'_>) -> Result<Self> {
        let data = file::read_binary_asset(file_name).await?;
        Self::new_cube_from_mem(gfx, &data)
    }

    pub fn view(&self) -> &wgpu::TextureView {
        &self.view
    }

    pub fn sampler(&self) -> &wgpu::Sampler {
        &self.sampler
    }

    pub fn format(&self) -> wgpu::TextureFormat {
        self.format
    }

    fn new_2d_from_mem(gfx: &Graphics, data: &[u8]) -> Result<Self> {
        let img = image::load_from_memory(data)?;
        let rgba = img.to_rgba8();
        let dimensions = img.dimensions();
        let size = wgpu::Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            depth_or_array_layers: 1,
        };
        let format = wgpu::TextureFormat::Rgba8UnormSrgb;

        let texture = gfx.create_texture_with_data(
            gfx.queue(),
            &wgpu::TextureDescriptor {
                label: None,
                size,
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format,
                usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                view_formats: &[],
            },
            TextureDataOrder::default(),
            &rgba,
        );

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = gfx.create_sampler(&new_sampler_descriptor(
            wgpu::FilterMode::Nearest,
            wgpu::FilterMode::Nearest,
            None,
        ));

        Ok(Self {
            view,
            sampler,
            format,
        })
    }

    fn new_cube_from_mem(gfx: &Graphics, data: &[u8]) -> Result<Self> {
        let image = ddsfile::Dds::read(&mut std::io::Cursor::new(&data)).unwrap();

        let format = wgpu::TextureFormat::Rgba8UnormSrgb; // TODO Configurable

        let size = wgpu::Extent3d {
            width: 128,
            height: 128,
            depth_or_array_layers: 6,
        };

        let layer_size = wgpu::Extent3d {
            depth_or_array_layers: 1,
            ..size
        };

        let texture = gfx.create_texture_with_data(
            gfx.queue(),
            &wgpu::TextureDescriptor {
                size,
                mip_level_count: layer_size.max_mips(wgpu::TextureDimension::D2),
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format,
                usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                label: None,
                view_formats: &[],
            },
            TextureDataOrder::default(),
            &image.data,
        );

        let view = texture.create_view(&wgpu::TextureViewDescriptor {
            label: None,
            dimension: Some(wgpu::TextureViewDimension::Cube),
            ..wgpu::TextureViewDescriptor::default()
        });

        let sampler = gfx.create_sampler(&new_sampler_descriptor(
            wgpu::FilterMode::Linear,
            wgpu::FilterMode::Linear,
            None,
        ));

        Ok(Self {
            view,
            sampler,
            format,
        })
    }
}

fn new_sampler_descriptor<'a>(
    filter: wgpu::FilterMode,
    mipmap_filter: wgpu::FilterMode,
    compare: Option<wgpu::CompareFunction>,
) -> wgpu::SamplerDescriptor<'a> {
    wgpu::SamplerDescriptor {
        label: None,
        address_mode_u: wgpu::AddressMode::ClampToEdge,
        address_mode_v: wgpu::AddressMode::ClampToEdge,
        address_mode_w: wgpu::AddressMode::ClampToEdge,
        mag_filter: filter,
        min_filter: filter,
        mipmap_filter,
        lod_min_clamp: 0.0,
        lod_max_clamp: 100.0,
        compare,
        ..Default::default()
    }
}
use anyhow::*;
use image::GenericImageView;
use wgpu::util::DeviceExt;
use crate::driver::Driver;
use crate::resources::load_binary;

pub struct Texture {
    _texture: wgpu::Texture,
    view: wgpu::TextureView,
    sampler: wgpu::Sampler,
}

impl Texture {
    pub const DEPTH_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;

    pub fn view(&self) -> &wgpu::TextureView { &self.view }
    pub fn sampler(&self) -> &wgpu::Sampler { &self.sampler }

    pub fn depth(driver: &Driver) -> Self {
        let size = wgpu::Extent3d {
            width: driver.surface_size().width,
            height: driver.surface_size().height,
            depth_or_array_layers: 1,
        };
        let desc = wgpu::TextureDescriptor {
            label: None,
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: Self::DEPTH_FORMAT,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[Self::DEPTH_FORMAT],
        };
        let texture = driver.device().create_texture(&desc);

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = driver.device().create_sampler(
            &wgpu::SamplerDescriptor {
                address_mode_u: wgpu::AddressMode::ClampToEdge,
                address_mode_v: wgpu::AddressMode::ClampToEdge,
                address_mode_w: wgpu::AddressMode::ClampToEdge,
                mag_filter: wgpu::FilterMode::Linear,
                min_filter: wgpu::FilterMode::Linear,
                mipmap_filter: wgpu::FilterMode::Nearest,
                compare: Some(wgpu::CompareFunction::LessEqual),
                lod_min_clamp: 0.0,
                lod_max_clamp: 100.0,
                ..Default::default()
            }
        );

        Self { _texture: texture, view, sampler }
    }

    pub async fn from_file_2d(file_name: &str, driver: &Driver) -> Result<Self> {
        let data = load_binary(file_name).await?;
        Self::from_mem_2d(driver, &data)
    }

    pub async fn from_file_cube(file_name: &str, driver: &Driver) -> Result<Self> {
        let data = load_binary(file_name).await?;
        Self::from_mem_cube(driver, &data)
    }

    fn from_mem_2d(driver: &Driver, bytes: &[u8]) -> Result<Self> {
        let img = image::load_from_memory(bytes)?;
        let rgba = img.to_rgba8();
        let dimensions = img.dimensions();
        let size = wgpu::Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            depth_or_array_layers: 1,
        };
        let format = wgpu::TextureFormat::Rgba8UnormSrgb;

        let texture = driver.device().create_texture_with_data(
            driver.queue(),
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
            &rgba
        );

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = driver.device().create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        Ok(Self {
            _texture: texture,
            view,
            sampler,
        })
    }

    fn from_mem_cube(driver: &Driver, bytes: &[u8]) -> Result<Self> {
        let image = ddsfile::Dds::read(&mut std::io::Cursor::new(&bytes)).unwrap();

        let format = wgpu::TextureFormat::Rgba8UnormSrgb;

        let size = wgpu::Extent3d {
            width: 128,
            height: 128,
            depth_or_array_layers: 6,
        };

        let layer_size = wgpu::Extent3d {
            depth_or_array_layers: 1,
            ..size
        };
        let max_mips = layer_size.max_mips(wgpu::TextureDimension::D2);

        let texture = driver.device().create_texture_with_data(
            driver.queue(),
            &wgpu::TextureDescriptor {
                size,
                mip_level_count: max_mips,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format,
                usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                label: None,
                view_formats: &[],
            },
            &image.data
        );

        let view = texture.create_view(&wgpu::TextureViewDescriptor {
            label: None,
            dimension: Some(wgpu::TextureViewDimension::Cube),
            ..wgpu::TextureViewDescriptor::default()
        });

        let sampler = driver.device().create_sampler(&wgpu::SamplerDescriptor {
            label: None,
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });

        Ok(Texture {
            _texture: texture,
            view,
            sampler
        })
    }
}

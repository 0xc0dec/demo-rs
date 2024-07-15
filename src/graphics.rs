use std::ops::Deref;

use wgpu::{Gles3MinorVersion, InstanceFlags};

use crate::texture::Texture;

pub type SurfaceSize = winit::dpi::PhysicalSize<u32>;

pub struct Graphics {
    surface: wgpu::Surface,
    surface_config: wgpu::SurfaceConfiguration,
    device: wgpu::Device,
    queue: wgpu::Queue,
    depth_tex: Texture,
}

impl Graphics {
    // TODO Configurable?
    const DEPTH_TEX_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;

    pub async fn new(window: &winit::window::Window) -> Self {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            dx12_shader_compiler: Default::default(),
            flags: InstanceFlags::DEBUG,
            gles_minor_version: Gles3MinorVersion::Automatic,
        });

        let surface = unsafe { instance.create_surface(&window) }.unwrap();

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                },
                None,
            )
            .await
            .unwrap();

        let surface_size = window.inner_size();

        let surface_config = {
            let caps = surface.get_capabilities(&adapter);

            let format = caps
                .formats
                .iter()
                .copied()
                .find(|f| f.is_srgb())
                .unwrap_or(caps.formats[0]);

            wgpu::SurfaceConfiguration {
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                format,
                width: surface_size.width,
                height: surface_size.height,
                present_mode: caps.present_modes[0],
                alpha_mode: caps.alpha_modes[0],
                view_formats: vec![],
            }
        };
        surface.configure(&device, &surface_config);

        let depth_tex = Texture::new_depth(&device, Self::DEPTH_TEX_FORMAT, surface_size.into());

        Self {
            surface_config,
            surface,
            device,
            queue,
            depth_tex,
        }
    }

    pub fn resize(&mut self, new_size: SurfaceSize) {
        if new_size.width > 0 && new_size.height > 0 {
            self.surface_config.width = new_size.width;
            self.surface_config.height = new_size.height;
            self.surface.configure(&self.device, &self.surface_config);
            self.depth_tex =
                Texture::new_depth(&self.device, Self::DEPTH_TEX_FORMAT, new_size.into());
        }
    }

    pub fn surface_texture_format(&self) -> wgpu::TextureFormat {
        self.surface_config.format
    }

    pub fn depth_texture_format(&self) -> wgpu::TextureFormat {
        Self::DEPTH_TEX_FORMAT
    }

    pub fn surface_size(&self) -> SurfaceSize {
        SurfaceSize::new(self.surface_config.width, self.surface_config.height)
    }

    pub fn queue(&self) -> &wgpu::Queue {
        &self.queue
    }

    pub fn surface(&self) -> &wgpu::Surface {
        &self.surface
    }

    pub fn depth_tex(&self) -> &Texture {
        &self.depth_tex
    }
}

impl Deref for Graphics {
    type Target = wgpu::Device;

    fn deref(&self) -> &Self::Target {
        &self.device
    }
}

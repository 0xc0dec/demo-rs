use crate::debug_ui::DebugUI;
use crate::render_target::RenderTarget;
use crate::texture::Texture;
use std::iter;
use std::ops::{Deref, DerefMut};

pub type SurfaceSize = winit::dpi::PhysicalSize<u32>;

// TODO Replace pub's with getters
pub struct Device {
    pub surface: wgpu::Surface,
    surface_config: wgpu::SurfaceConfiguration,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    // TODO Make it non-optional. Currently it is just because Texture's ctor requires
    // a reference to Device, which we cannot provide before we have constructed the device.
    pub depth_tex: Option<Texture>,
}

impl Device {
    // TODO Configurable?
    const DEPTH_TEX_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;

    pub async fn new(window: &winit::window::Window) -> Self {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            dx12_shader_compiler: Default::default(),
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
                .filter(|f| f.describe().srgb)
                .next()
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

        Self {
            surface_config,
            surface,
            device,
            queue,
            depth_tex: None,
        }
    }

    pub fn new_frame<'a>(&'a self, target: Option<&'a RenderTarget>) -> Frame<'a> {
        let (color_format, depth_format) = match target {
            Some(ref target) => (target.color_tex().format(), target.depth_tex().format()),
            None => (
                self.surface_config.format,
                self.depth_tex.as_ref().unwrap().format(),
            ),
        };

        let bundle_encoder =
            self.device
                .create_render_bundle_encoder(&wgpu::RenderBundleEncoderDescriptor {
                    label: None,
                    multiview: None,
                    sample_count: 1,
                    color_formats: &[Some(color_format)],
                    depth_stencil: Some(wgpu::RenderBundleDepthStencil {
                        format: depth_format,
                        depth_read_only: false,
                        stencil_read_only: false,
                    }),
                });

        Frame {
            device: &self,
            bundle_encoder,
            target,
        }
    }

    pub fn resize(&mut self, new_size: SurfaceSize) {
        if new_size.width > 0 && new_size.height > 0 {
            self.surface_config.width = new_size.width;
            self.surface_config.height = new_size.height;
            self.surface.configure(&self.device, &self.surface_config);
            self.depth_tex = Some(Texture::new_depth(&self, new_size.into()));
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

    pub fn device(&self) -> &wgpu::Device {
        &self.device
    }

    pub fn queue(&self) -> &wgpu::Queue {
        &self.queue
    }
}

pub struct Frame<'a>
{
    device: &'a Device,
    bundle_encoder: wgpu::RenderBundleEncoder<'a>,
    target: Option<&'a RenderTarget>,
}

impl<'a> Deref for Frame<'a>
{
    type Target = wgpu::RenderBundleEncoder<'a>;

    fn deref(&self) -> &Self::Target {
        &self.bundle_encoder
    }
}

impl<'a> DerefMut for Frame<'a>
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.bundle_encoder
    }
}

impl<'a> Frame<'a> {
    pub fn finish(self, debug_ui: Option<&mut DebugUI>) {
        let surface_tex = self.target.is_none().then(|| {
            self.device
                .surface
                .get_current_texture()
                .expect("Missing surface texture")
        });
        let surface_tex_view = surface_tex.as_ref().map(|t| {
            t.texture
                .create_view(&wgpu::TextureViewDescriptor::default())
        });

        let color_tex_view = self
            .target
            .map(|t| t.color_tex().view())
            .or(surface_tex_view.as_ref())
            .unwrap();
        let color_attachment = Some(wgpu::RenderPassColorAttachment {
            view: color_tex_view,
            resolve_target: None,
            ops: wgpu::Operations {
                load: wgpu::LoadOp::Clear(wgpu::Color::RED),
                store: true,
            },
        });

        let depth_tex_view = self
            .target
            .map(|t| t.depth_tex().view())
            .or(self.device.depth_tex.as_ref().map(|t| t.view()))
            .unwrap();
        let depth_attachment = Some(wgpu::RenderPassDepthStencilAttachment {
            view: depth_tex_view,
            depth_ops: Some(wgpu::Operations {
                load: wgpu::LoadOp::Clear(1.0),
                store: true,
            }),
            stencil_ops: None,
        });

        let bundle = self
            .bundle_encoder
            .finish(&wgpu::RenderBundleDescriptor { label: None });

        let cmd_buffer = {
            let mut encoder = self
                .device
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

            {
                let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: None,
                    color_attachments: &[color_attachment],
                    depth_stencil_attachment: depth_attachment,
                });

                pass.execute_bundles(iter::once(&bundle));

                debug_ui.map(|ui| ui.render(self.device, &mut pass));
            }

            encoder.finish()
        };

        self.device.queue.submit(Some(cmd_buffer));

        surface_tex.map(|t| t.present());
    }
}

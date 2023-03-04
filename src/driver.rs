use std::iter;
use wgpu::{Device, Queue, TextureFormat};
use winit::dpi::PhysicalSize;
use winit::window::Window;
use crate::render_target::RenderTarget;
use crate::scene::Scene;

pub struct Driver {
    surface_size: PhysicalSize<u32>,
    surface: wgpu::Surface,
    device: Device,
    queue: Queue,
    surface_config: wgpu::SurfaceConfiguration,
}

impl Driver {
    pub async fn new(window: &Window) -> Driver {
        let surface_size = window.inner_size();

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            dx12_shader_compiler: Default::default()
        });

        let surface = unsafe { instance.create_surface(&window) }.unwrap();

        let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            compatible_surface: Some(&surface),
            force_fallback_adapter: false
        }).await.unwrap();

        let (device, queue) = adapter.request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                features: wgpu::Features::empty(),
                limits: wgpu::Limits::default()
            },
            None,
        ).await.unwrap();

        let surface_config = {
            let caps = surface.get_capabilities(&adapter);

            let format = caps.formats.iter()
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

        Driver {
            surface_size,
            surface,
            device,
            queue,
            surface_config,
        }
    }

    pub fn resize(&mut self, new_size: Option<PhysicalSize<u32>>) {
        let size = new_size.unwrap_or(self.surface_size);
        if size.width > 0 && size.height > 0 {
            self.surface_size = size;
            self.surface_config.width = size.width;
            self.surface_config.height = size.height;
            self.surface.configure(&self.device, &self.surface_config);
        }
    }

    pub fn render_frame(&self, target: &RenderTarget, scene: &mut Scene) {
        let output = self.surface.get_current_texture().expect("Missing surface texture");
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: None
        });

        {
            let mut rp = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.0,
                            g: 0.5,
                            b: 0.0,
                            a: 1.0,
                        }),
                        store: true,
                    }
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: target.depth_texture().view(),
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: true,
                    }),
                    stencil_ops: None,
                })
            });

            scene.render(&self, &mut rp);
        }

        self.queue().submit(iter::once(encoder.finish()));
        output.present();
    }

    pub fn surface_texture_format(&self) -> TextureFormat { self.surface_config.format }
    pub fn surface_size(&self) -> PhysicalSize<u32> { self.surface_size }
    pub fn device(&self) -> &Device { &self.device }
    pub fn queue(&self) -> &Queue { &self.queue }
}
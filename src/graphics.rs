use std::iter;
use crate::render_target::RenderTarget;
use crate::texture::Texture;

pub struct Graphics {
    surface_size: winit::dpi::PhysicalSize<u32>,
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    surface_config: wgpu::SurfaceConfiguration,
    depth_tex: Option<Texture>,
}

impl Graphics {
    pub async fn new(window: &winit::window::Window) -> Self {
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

        Self {
            surface_size,
            surface,
            device,
            queue,
            surface_config,
            depth_tex: None
        }
    }

    pub fn new_render_encoder(&self, target: Option<&RenderTarget>) -> wgpu::RenderBundleEncoder {
        let (color_format, depth_format) = if let Some(ref target) = target {
            (target.color_tex().format(), target.depth_tex().format())
        } else {
            (self.surface_config.format, self.depth_tex.as_ref().unwrap().format())
        };

        self.device.create_render_bundle_encoder(
            &wgpu::RenderBundleEncoderDescriptor {
                label: None,
                multiview: None,
                sample_count: 1,
                color_formats: &[Some(color_format)],
                depth_stencil: Some(wgpu::RenderBundleDepthStencil {
                    format: depth_format,
                    depth_read_only: false,
                    stencil_read_only: false
                })
            }
        )
    }

    pub fn render_to_target(&mut self, target: &RenderTarget, bundle: wgpu::RenderBundle) {
        let cmd_buffer = {
            let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: None
            });

            {
                let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: None,
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: target.color_tex().view(),
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color {
                                r: 1.0,
                                g: 1.0,
                                b: 1.0,
                                a: 1.0
                            }),
                            store: true,
                        }
                    })],
                    depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                        view: target.depth_tex().view(),
                        depth_ops: Some(wgpu::Operations {
                            load: wgpu::LoadOp::Clear(1.0),
                            store: true,
                        }),
                        stencil_ops: None,
                    })
                });

                pass.execute_bundles(iter::once(&bundle));
            }

            encoder.finish()
        };

        self.queue.submit(Some(cmd_buffer));
    }

    pub fn render_to_surface(&self, bundle: wgpu::RenderBundle) {
        let target_tex = self.surface.get_current_texture().expect("Missing surface texture");

        let cmd_buffer = {
            let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: None
            });

            {
                let target_tex_view = target_tex.texture.create_view(&wgpu::TextureViewDescriptor::default());
                let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: None,
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &target_tex_view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color {
                                r: 1.0,
                                g: 1.0,
                                b: 1.0,
                                a: 1.0
                            }),
                            store: true,
                        }
                    })],
                    depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                        view: self.depth_tex.as_ref().unwrap().view(),
                        depth_ops: Some(wgpu::Operations {
                            load: wgpu::LoadOp::Clear(1.0),
                            store: true,
                        }),
                        stencil_ops: None,
                    })

                });

                pass.execute_bundles(iter::once(&bundle));
            }

            encoder.finish()
        };

        self.queue.submit(Some(cmd_buffer));
        target_tex.present();
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.surface_size = new_size;
            self.surface_config.width = new_size.width;
            self.surface_config.height = new_size.height;
            self.surface.configure(&self.device, &self.surface_config);
            self.depth_tex = Some(Texture::new_depth(&self, self.surface_size));
        }
    }

    pub fn surface_texture_format(&self) -> wgpu::TextureFormat { self.surface_config.format }
    pub fn surface_size(&self) -> winit::dpi::PhysicalSize<u32> { self.surface_size }
    pub fn device(&self) -> &wgpu::Device { &self.device }
    pub fn queue(&self) -> &wgpu::Queue { &self.queue }
}
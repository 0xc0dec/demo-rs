use std::iter;
use wgpu::{Device, Queue, Surface, SurfaceConfiguration, TextureFormat};
use winit::dpi::PhysicalSize;
use winit::window::Window;
use crate::frame_context::FrameContext;
use crate::materials::{Material, PostProcessMaterial};
use crate::model::{DrawModel, Mesh};
use crate::state::State;
use crate::texture::Texture;

pub struct Graphics {
    surface_size: PhysicalSize<u32>,
    surface: Surface,
    device: Device,
    queue: Queue,
    surface_config: SurfaceConfiguration,
    depth_tex: Option<Texture>,
}

impl Graphics {
    pub async fn new(window: &Window) -> Graphics {
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

            SurfaceConfiguration {
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

        Graphics {
            surface_size,
            surface,
            device,
            queue,
            surface_config,
            depth_tex: None
        }
    }

    pub fn render_to_target(&mut self, target: &Texture, state: &mut State, ctx: &mut FrameContext) {
        if let Some(new_size) = ctx.events.new_surface_size {
            self.resize(new_size);
        }

        let cmd_buffer = {
            let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: None
            });

            {
                let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: None,
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: target.view(),
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

                state.render(&self, &mut pass, ctx);
            }

            encoder.finish()
        };

        self.queue.submit(iter::once(cmd_buffer));
    }

    pub fn render_post_process(&mut self, material: &mut PostProcessMaterial, quad: &Mesh, ctx: &mut FrameContext) {
        if let Some(new_size) = ctx.events.new_surface_size {
            self.resize(new_size);
        }

        let output = self.surface.get_current_texture().expect("Missing surface texture");

        let cmd_buffer = {
            let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: None
            });

            {
                let output_view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
                let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: None,
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &output_view,
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
                    depth_stencil_attachment: None
                });

                material.apply(&mut pass);
                pass.draw_mesh(quad);
            }

            encoder.finish()
        };

        self.queue.submit(iter::once(cmd_buffer));
        output.present();
    }

    fn resize(&mut self, new_size: PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.surface_size = new_size;
            self.surface_config.width = new_size.width;
            self.surface_config.height = new_size.height;
            self.surface.configure(&self.device, &self.surface_config);
            self.depth_tex = Some(Texture::new_depth(&self));
        }
    }

    pub fn surface_texture_format(&self) -> TextureFormat { self.surface_config.format }
    pub fn surface_size(&self) -> PhysicalSize<u32> { self.surface_size }
    pub fn device(&self) -> &Device { &self.device }
    pub fn queue(&self) -> &Queue { &self.queue }
}
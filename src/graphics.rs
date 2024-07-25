use std::ops::Deref;
use std::sync::Arc;

use wgpu::{Gles3MinorVersion, InstanceFlags, RenderBundle, StoreOp};

use crate::camera::Camera;
use crate::materials::Material;
use crate::mesh::{DrawMesh, Mesh};
use crate::render_target::RenderTarget;
use crate::texture::Texture;
use crate::transform::Transform;

pub type SurfaceSize = winit::dpi::PhysicalSize<u32>;

pub struct Graphics<'a> {
    surface: wgpu::Surface<'a>,
    surface_config: wgpu::SurfaceConfiguration,
    device: wgpu::Device,
    queue: wgpu::Queue,
    depth_tex: Texture,
}

impl<'a> Graphics<'a> {
    // TODO Configurable?
    const DEPTH_TEX_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;

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

    pub async fn new(window: Arc<winit::window::Window>) -> Graphics<'a> {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::PRIMARY,
            dx12_shader_compiler: Default::default(),
            flags: InstanceFlags::DEBUG,
            gles_minor_version: Gles3MinorVersion::Automatic,
        });

        let surface = instance.create_surface(Arc::clone(&window)).unwrap();

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
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::default(),
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
                desired_maximum_frame_latency: 2,
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

    pub fn build_render_bundle(
        &self,
        mesh: &Mesh,
        material: &mut dyn Material,
        transform: &Transform,
        camera: (&Camera, &Transform),
    ) -> RenderBundle {
        let mut encoder = self.new_bundle_encoder(camera.0.target().as_ref());
        material.apply(&mut encoder, self, camera, transform);
        encoder.draw_mesh(mesh);
        encoder.finish(&wgpu::RenderBundleDescriptor { label: None })
    }

    pub fn render_pass(&self, bundles: &[RenderBundle], target: Option<&RenderTarget>) {
        let surface_tex = target.is_none().then(|| {
            self.surface
                .get_current_texture()
                .expect("Missing surface texture")
        });
        let surface_tex_view = surface_tex.as_ref().map(|t| {
            t.texture
                .create_view(&wgpu::TextureViewDescriptor::default())
        });

        let color_tex_view = target
            .map(|t| t.color_tex().view())
            .or(surface_tex_view.as_ref())
            .unwrap();
        let color_attachment = Some(wgpu::RenderPassColorAttachment {
            view: color_tex_view,
            resolve_target: None,
            ops: wgpu::Operations {
                load: wgpu::LoadOp::Clear(wgpu::Color::RED),
                store: StoreOp::Store,
            },
        });

        let depth_tex_view = target
            .map(|t| t.depth_tex().view())
            .unwrap_or(self.depth_tex.view());
        let depth_attachment = Some(wgpu::RenderPassDepthStencilAttachment {
            view: depth_tex_view,
            depth_ops: Some(wgpu::Operations {
                load: wgpu::LoadOp::Clear(1.0),
                store: StoreOp::Store,
            }),
            stencil_ops: None,
        });

        let cmd_buffer = {
            let mut encoder =
                self.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

            {
                let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: None,
                    color_attachments: &[color_attachment],
                    depth_stencil_attachment: depth_attachment,
                    occlusion_query_set: None,
                    timestamp_writes: None,
                });

                pass.execute_bundles(bundles.iter());
            }

            encoder.finish()
        };

        self.queue.submit(Some(cmd_buffer));
        if let Some(t) = surface_tex {
            t.present()
        }
    }

    fn new_bundle_encoder(&self, target: Option<&RenderTarget>) -> wgpu::RenderBundleEncoder {
        let color_format = target.map_or(self.surface_texture_format(), |t| t.color_tex().format());
        let depth_format = target.map_or(self.depth_texture_format(), |t| t.depth_tex().format());

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
            })
    }
}

impl<'a> Deref for Graphics<'a> {
    type Target = wgpu::Device;

    fn deref(&self) -> &Self::Target {
        &self.device
    }
}

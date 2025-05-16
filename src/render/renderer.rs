use std::ops::Deref;
use std::sync::Arc;
use wgpu::util::DeviceExt;
use wgpu::{BackendOptions, Trace};

use super::render_target::RenderTarget;
use super::texture::Texture;
use crate::render::DrawMesh;
use crate::scene::materials::{ApplyMaterial, Material};
use crate::scene::{Assets, MaterialHandle, MeshHandle};
use crate::ui::Ui;

pub type SurfaceSize = winit::dpi::PhysicalSize<u32>;

pub struct RenderPipelineParams<'a> {
    pub shader_module: &'a wgpu::ShaderModule,
    pub depth_write: bool,
    pub depth_enabled: bool,
    pub bind_group_layouts: &'a [&'a wgpu::BindGroupLayout],
    pub vertex_buffer_layouts: &'a [wgpu::VertexBufferLayout<'a>],
}

pub struct Renderer<'a> {
    surface: wgpu::Surface<'a>,
    surface_config: wgpu::SurfaceConfiguration,
    device: wgpu::Device,
    queue: wgpu::Queue,
    depth_tex: Texture,
}

impl<'a> Renderer<'a> {
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

    pub async fn new(window: Arc<winit::window::Window>) -> Renderer<'a> {
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends: wgpu::Backends::PRIMARY,
            flags: wgpu::InstanceFlags::DEBUG,
            backend_options: BackendOptions::default(),
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
            .request_device(&wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
                memory_hints: wgpu::MemoryHints::default(),
                trace: Trace::Off,
            })
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

    pub fn update(&mut self, new_surface_size: Option<SurfaceSize>) {
        if let Some(SurfaceSize { width, height }) = new_surface_size {
            if width > 0 && height > 0 {
                self.surface_config.width = width;
                self.surface_config.height = height;
                self.surface.configure(&self.device, &self.surface_config);
                self.depth_tex =
                    Texture::new_depth(&self.device, Self::DEPTH_TEX_FORMAT, (width, height));
            }
        }
    }

    pub fn build_render_bundle(
        &self,
        mesh: MeshHandle,
        material: MaterialHandle,
        rt: Option<&RenderTarget>,
        assets: &Assets,
    ) -> wgpu::RenderBundle {
        let mut encoder = self.new_bundle_encoder(rt);
        match assets.material(material) {
            Material::Color(m) => m.apply(&mut encoder),
            Material::Skybox(m) => m.apply(&mut encoder),
            Material::Textured(m) => m.apply(&mut encoder),
            Material::PostProcess(m) => m.apply(&mut encoder),
        }
        encoder.draw_mesh(assets.mesh(mesh));
        encoder.finish(&wgpu::RenderBundleDescriptor { label: None })
    }

    pub fn render_pass(
        &self,
        bundles: &[wgpu::RenderBundle],
        target: Option<&RenderTarget>,
        ui: Option<&mut Ui>,
    ) {
        let surface_tex = target.is_none().then(|| {
            self.surface
                .get_current_texture()
                .expect("Missing surface texture")
        });
        let surface_tex_view = surface_tex.as_ref().map(|t| {
            t.texture
                .create_view(&wgpu::TextureViewDescriptor::default())
        });

        let color_attachment = Some(wgpu::RenderPassColorAttachment {
            view: target
                .map(|t| t.color_tex().view())
                .or(surface_tex_view.as_ref())
                .unwrap(),
            resolve_target: None,
            ops: wgpu::Operations {
                load: wgpu::LoadOp::Clear(wgpu::Color::RED),
                store: wgpu::StoreOp::Store,
            },
        });

        let depth_attachment = Some(wgpu::RenderPassDepthStencilAttachment {
            view: target
                .map(|t| t.depth_tex().view())
                .unwrap_or(self.depth_tex.view()),
            depth_ops: Some(wgpu::Operations {
                load: wgpu::LoadOp::Clear(1.0),
                store: wgpu::StoreOp::Store,
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
                if let Some(ui) = ui {
                    ui.render(self, &mut pass);
                }
            }

            encoder.finish()
        };

        self.queue.submit(Some(cmd_buffer));

        if let Some(t) = surface_tex {
            t.present()
        }
    }

    pub fn new_uniform_bind_group(
        &self,
        data: &[u8],
    ) -> (wgpu::BindGroupLayout, wgpu::BindGroup, wgpu::Buffer) {
        let buffer = self.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: data,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let layout = self.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
            label: None,
        });

        let group = self.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            }],
            label: None,
        });

        (layout, group, buffer)
    }

    pub fn new_texture_bind_group(
        &self,
        texture: &Texture,
        view_dimension: wgpu::TextureViewDimension,
    ) -> (wgpu::BindGroupLayout, wgpu::BindGroup) {
        let layout = self.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        multisampled: false,
                        view_dimension,
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
            label: None,
        });

        let group = self.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(texture.view()),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(texture.sampler()),
                },
            ],
            label: None,
        });

        (layout, group)
    }

    pub fn new_render_pipeline(&self, params: RenderPipelineParams<'_>) -> wgpu::RenderPipeline {
        let layout = self.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: params.bind_group_layouts,
            push_constant_ranges: &[],
        });

        self.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&layout),
            vertex: wgpu::VertexState {
                module: params.shader_module,
                entry_point: Some("vs_main"),
                buffers: params.vertex_buffer_layouts,
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: params.shader_module,
                entry_point: Some("fs_main"),
                compilation_options: Default::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format: self.surface_config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None,
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: if params.depth_enabled {
                Some(wgpu::DepthStencilState {
                    format: Self::DEPTH_TEX_FORMAT,
                    depth_write_enabled: params.depth_write,
                    depth_compare: wgpu::CompareFunction::Less,
                    stencil: wgpu::StencilState::default(),
                    bias: wgpu::DepthBiasState::default(),
                })
            } else {
                None
            },
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
            cache: None,
        })
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

impl Deref for Renderer<'_> {
    type Target = wgpu::Device;

    fn deref(&self) -> &Self::Target {
        &self.device
    }
}

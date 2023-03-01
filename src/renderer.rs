use wgpu::{Device, Queue, TextureFormat};
use winit::dpi::PhysicalSize;
use winit::window::Window;

pub struct Renderer {
    canvas_size: PhysicalSize<u32>,
    surface: wgpu::Surface,
    device: Device,
    queue: Queue,
    surface_config: wgpu::SurfaceConfiguration,
}

impl Renderer {
    pub async fn new(window: &Window) -> Renderer {
        let canvas_size = window.inner_size();

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
                width: canvas_size.width,
                height: canvas_size.height,
                present_mode: caps.present_modes[0],
                alpha_mode: caps.alpha_modes[0],
                view_formats: vec![],
            }
        };
        surface.configure(&device, &surface_config);

        Renderer {
            canvas_size,
            surface,
            device,
            queue,
            surface_config,
        }
    }

    pub fn resize(&mut self, new_size: Option<PhysicalSize<u32>>) {
        let size = new_size.unwrap_or(self.canvas_size);
        if size.width > 0 && size.height > 0 {
            self.canvas_size = size;
            self.surface_config.width = size.width;
            self.surface_config.height = size.height;
            self.surface.configure(&self.device, &self.surface_config);
        }
    }

    pub fn surface(&self) -> &wgpu::Surface { &self.surface }
    pub fn surface_texture_format(&self) -> TextureFormat { self.surface_config.format }
    pub fn device(&self) -> &Device { &self.device }
    pub fn queue(&self) -> &Queue { &self.queue }
    pub fn canvas_size(&self) -> PhysicalSize<u32> { self.canvas_size }
}
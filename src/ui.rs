use imgui::{Context, FontSource, MouseCursor};
use imgui_wgpu::{Renderer, RendererConfig};
use imgui_winit_support::WinitPlatform;
use std::time::Instant;
use wgpu::{Device, Queue, TextureFormat};
use winit::window::Window;

pub struct Ui {
    context: Context,
    platform: WinitPlatform,
    renderer: Renderer,
    clear_color: wgpu::Color,
    demo_open: bool,
    last_frame: Instant,
    last_cursor: Option<MouseCursor>,
}

impl Ui {
    pub fn new(
        device: &Device,
        queue: &Queue,
        window: &Window,
        hidpi_factor: f64,
        texture_format: TextureFormat,
    ) -> Self {
        let mut context = imgui::Context::create();

        let mut platform = WinitPlatform::new(&mut context);
        platform.attach_window(
            context.io_mut(),
            window,
            imgui_winit_support::HiDpiMode::Default,
        );
        context.set_ini_filename(None);

        let font_size = (13.0 * hidpi_factor) as f32;
        context.io_mut().font_global_scale = (1.0 / hidpi_factor) as f32;

        context.fonts().add_font(&[FontSource::DefaultFontData {
            config: Some(imgui::FontConfig {
                oversample_h: 1,
                pixel_snap_h: true,
                size_pixels: font_size,
                ..Default::default()
            }),
        }]);

        let clear_color = wgpu::Color {
            r: 0.1,
            g: 0.2,
            b: 0.3,
            a: 1.0,
        };

        let renderer_config = RendererConfig {
            texture_format,
            ..Default::default()
        };

        let renderer = Renderer::new(&mut context, device, &queue, renderer_config);
        let last_frame = Instant::now();
        let last_cursor = None;
        let demo_open = true;

        Ui {
            context,
            platform,
            renderer,
            clear_color,
            demo_open,
            last_frame,
            last_cursor,
        }
    }
}

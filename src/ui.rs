use imgui::{Context, FontSource, MouseCursor};
use imgui_wgpu::{Renderer, RendererConfig};
use imgui_winit_support::WinitPlatform;
use std::time::Duration;
use wgpu::{Device, Queue, RenderPass, TextureFormat};
use winit::window::Window;

pub struct Ui {
    context: Context,
    platform: WinitPlatform,
    renderer: Renderer,
    demo_open: bool,
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

        let renderer_config = RendererConfig {
            texture_format,
            ..Default::default()
        };

        let renderer = Renderer::new(&mut context, device, &queue, renderer_config);
        let last_cursor = None;
        let demo_open = true;

        Ui {
            context,
            platform,
            renderer,
            demo_open,
            last_cursor,
        }
    }

    pub fn new_frame(&mut self, dt: f32, window: &Window, build: impl FnOnce(&mut imgui::Ui)) {
        self.context
            .io_mut()
            .update_delta_time(Duration::from_secs_f32(dt)); // TODO Avoid the conversion.
        self.platform
            .prepare_frame(self.context.io_mut(), window)
            .expect("Failed to prepare UI frame");
        let frame = self.context.new_frame();
        build(frame);

        if self.last_cursor != frame.mouse_cursor() {
            self.last_cursor = frame.mouse_cursor();
            self.platform.prepare_render(frame, window);
        }
    }

    pub fn render<'a>(&'a mut self, device: &Device, queue: &Queue, pass: &mut RenderPass<'a>) {
        self.renderer
            .render(self.context.render(), queue, device, pass)
            .expect("Rendering failed");
    }
}

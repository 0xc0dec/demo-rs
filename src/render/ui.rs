use imgui::{Context, FontSource, MouseCursor};
use imgui_wgpu::RendererConfig;
use imgui_winit_support::WinitPlatform;
use std::time::Duration;
use winit::event::Event;

use super::renderer::Renderer;

// By the virtue of how the underlying library is implemented, this should be used as a singleton.
// Or I just don't know Rust enough to make it work :|
pub struct Ui {
    context: Context,
    platform: WinitPlatform,
    renderer: imgui_wgpu::Renderer,
    last_cursor: Option<MouseCursor>,
}

impl Ui {
    const FONT_SIZE: f64 = 13.0;

    pub fn new(window: &winit::window::Window, rr: &Renderer) -> Self {
        let mut context = Context::create();

        let mut platform = WinitPlatform::new(&mut context);
        platform.attach_window(
            context.io_mut(),
            window,
            imgui_winit_support::HiDpiMode::Default,
        );
        context.set_ini_filename(None);

        let font_size = (Self::FONT_SIZE * window.scale_factor()) as f32;
        context.io_mut().font_global_scale = (1.0 / window.scale_factor()) as f32;

        context.fonts().add_font(&[FontSource::DefaultFontData {
            config: Some(imgui::FontConfig {
                oversample_h: 1,
                pixel_snap_h: true,
                size_pixels: font_size,
                ..Default::default()
            }),
        }]);

        let renderer = imgui_wgpu::Renderer::new(
            &mut context,
            rr,
            rr.queue(),
            RendererConfig {
                texture_format: rr.surface_texture_format(),
                depth_format: Some(rr.depth_texture_format()),
                ..Default::default()
            },
        );
        let last_cursor = None;

        Ui {
            context,
            platform,
            renderer,
            last_cursor,
        }
    }

    pub fn prepare_frame(
        &mut self,
        dt: f32,
        window: &winit::window::Window,
        build: impl FnOnce(&mut imgui::Ui),
    ) {
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

    pub fn handle_event(&mut self, event: &Event<()>, window: &winit::window::Window) {
        self.platform
            .handle_event(self.context.io_mut(), window, event)
    }

    pub fn draw<'a>(&'a mut self, rr: &Renderer, pass: &mut wgpu::RenderPass<'a>) {
        self.renderer
            .render(self.context.render(), rr.queue(), rr, pass)
            .expect("Rendering UI failed");
    }
}

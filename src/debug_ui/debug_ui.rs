use std::time::Duration;

use imgui::MouseCursor;
use winit::event::Event;
use winit::window::Window;

use crate::debug_ui::imgui_winit;
use crate::graphics::Graphics;

pub struct DebugUI {
    renderer: imgui_wgpu::Renderer,
    context: imgui::Context,
    platform: imgui_winit::WinitPlatform,
    last_cursor: Option<MouseCursor>,
}

impl DebugUI {
    pub fn new(gfx: &Graphics, window: &Window) -> Self {
        let mut context = imgui::Context::create();
        let mut platform = imgui_winit::WinitPlatform::init(&mut context);
        platform.attach_window(context.io_mut(), window, imgui_winit::HiDpiMode::Default);
        context.set_ini_filename(None);

        let font_size = (13.0 * window.scale_factor()) as f32;
        context.io_mut().font_global_scale = (1.0 / window.scale_factor()) as f32;

        context
            .fonts()
            .add_font(&[imgui::FontSource::DefaultFontData {
                config: Some(imgui::FontConfig {
                    oversample_h: 1,
                    pixel_snap_h: true,
                    size_pixels: font_size,
                    ..Default::default()
                }),
            }]);

        let renderer_config = imgui_wgpu::RendererConfig {
            texture_format: gfx.surface_texture_format(),
            depth_format: Some(gfx.depth_texture_format()),
            ..Default::default()
        };

        let renderer = imgui_wgpu::Renderer::new(&mut context, gfx, gfx.queue(), renderer_config);

        Self {
            context,
            renderer,
            platform,
            last_cursor: None,
        }
    }

    pub fn handle_window_event(&mut self, window: &Window, event: &Event<()>) {
        self.platform
            .handle_event(self.context.io_mut(), window, event);
    }

    pub fn prepare_render(&mut self, window: &Window, dt: f32, build: impl Fn(&mut imgui::Ui)) {
        self.context
            .io_mut()
            .update_delta_time(Duration::from_secs_f32(dt));
        self.platform
            .prepare_frame(self.context.io_mut(), window)
            .expect("Failed to prepare debug UI frame");

        let frame = self.context.frame();

        build(frame);

        if self.last_cursor != frame.mouse_cursor() {
            self.last_cursor = frame.mouse_cursor();
            self.platform.prepare_render(frame, window);
        }
    }

    pub fn render<'a>(&'a mut self, gfx: &Graphics, pass: &mut wgpu::RenderPass<'a>) {
        let draw_data = self.context.render();
        self.renderer
            .render(draw_data, gfx.queue(), gfx, pass)
            .expect("Failed to render debug UI");
    }
}

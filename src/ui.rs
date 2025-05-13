use crate::renderer::Renderer;
use crate::state::State;
use imgui::{Context, FontSource, MouseCursor};
use imgui_wgpu::RendererConfig;
use imgui_winit_support::WinitPlatform;
use std::time::Duration;
use wgpu::{Device, Queue, RenderPass};
use winit::event::Event;
use winit::window::Window;

pub struct Ui {
    context: Context,
    platform: WinitPlatform,
    renderer: imgui_wgpu::Renderer,
    last_cursor: Option<MouseCursor>,
}

impl Ui {
    const FONT_SIZE: f64 = 13.0;

    pub fn new(rr: &Renderer, window: &Window, hidpi_factor: f64) -> Self {
        let mut context = Context::create();

        let mut platform = WinitPlatform::new(&mut context);
        platform.attach_window(
            context.io_mut(),
            window,
            imgui_winit_support::HiDpiMode::Default,
        );
        context.set_ini_filename(None);

        let font_size = (Self::FONT_SIZE * hidpi_factor) as f32;
        context.io_mut().font_global_scale = (1.0 / hidpi_factor) as f32;

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

    pub fn prepare_frame(&mut self, dt: f32, state: &State, build: impl FnOnce(&mut imgui::Ui)) {
        self.context
            .io_mut()
            .update_delta_time(Duration::from_secs_f32(dt)); // TODO Avoid the conversion.

        self.platform
            .prepare_frame(self.context.io_mut(), &state.window)
            .expect("Failed to prepare UI frame");

        let frame = self.context.new_frame();
        build(frame);

        if self.last_cursor != frame.mouse_cursor() {
            self.last_cursor = frame.mouse_cursor();
            self.platform.prepare_render(frame, &state.window);
        }
    }

    pub fn handle_event(&mut self, event: &Event<()>, state: &State) {
        self.platform
            .handle_event(self.context.io_mut(), &state.window, event)
    }

    pub fn render<'a>(&'a mut self, device: &Device, queue: &Queue, pass: &mut RenderPass<'a>) {
        self.renderer
            .render(self.context.render(), queue, device, pass)
            .expect("Rendering failed");
    }
}

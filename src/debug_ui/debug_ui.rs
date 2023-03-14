use std::time::Duration;
use imgui::{MouseCursor};
use wgpu::{RenderPass};
use winit::event::Event;
use winit::window::Window;
use crate::debug_ui::imgui_winit;
use crate::device::Device;
use crate::frame_context::FrameContext;

pub struct DebugUI {
    renderer: imgui_wgpu::Renderer,
    imgui: imgui::Context,
    platform: imgui_winit::WinitPlatform,
    last_cursor: Option<MouseCursor>,
}

impl DebugUI {
    pub fn new(device: &Device, window: &Window) -> Self {
        let mut imgui = imgui::Context::create();
        let mut platform = imgui_winit::WinitPlatform::init(&mut imgui);
        platform.attach_window(
            imgui.io_mut(),
            &window,
            imgui_winit::HiDpiMode::Default,
        );
        imgui.set_ini_filename(None);

        let font_size = (13.0 * window.scale_factor()) as f32;
        imgui.io_mut().font_global_scale = (1.0 / window.scale_factor()) as f32;

        imgui.fonts().add_font(&[imgui::FontSource::DefaultFontData {
            config: Some(imgui::FontConfig {
                oversample_h: 1,
                pixel_snap_h: true,
                size_pixels: font_size,
                ..Default::default()
            }),
        }]);

        let renderer_config = imgui_wgpu::RendererConfig {
            texture_format: device.surface_texture_format(),
            depth_format: Some(device.depth_texture_format()),
            ..Default::default()
        };

        let renderer = imgui_wgpu::Renderer::new(
            &mut imgui, device.device(), device.queue(), renderer_config
        );

        Self {
            imgui,
            renderer,
            platform,
            last_cursor: None
        }
    }

    pub fn handle_window_event(&mut self, window: &Window, event: &Event<()>) {
        self.platform.handle_event(self.imgui.io_mut(), &window, &event);
    }

    pub fn update(&mut self, ctx: &FrameContext) {
        self.imgui.io_mut().update_delta_time(Duration::from_secs_f32(ctx.dt));
    }

    pub fn render<'a>(&'a mut self, pass: &mut RenderPass<'a>, ctx: &FrameContext) {
        self.platform
            .prepare_frame(self.imgui.io_mut(), ctx.window)
            .expect("Failed to prepare frame");
        let frame = self.imgui.frame();

        // TODO Remove after testing
        {
            frame.window("Hello there!")
                .size([300.0, 100.0], imgui::Condition::FirstUseEver)
                .build(|| {
                    frame.text("Some test string.");
                    frame.separator();
                    let mouse_pos = frame.io().mouse_pos;
                    frame.text(format!(
                        "Mouse Position: ({:.1},{:.1})",
                        mouse_pos[0], mouse_pos[1]
                    ));
                });
        }

        if self.last_cursor != frame.mouse_cursor() {
            self.last_cursor = frame.mouse_cursor();
            self.platform.prepare_render(frame, ctx.window);
        }

        let draw_data = self.imgui.render();

        self.renderer
            .render(draw_data, ctx.device.queue(), ctx.device.device(), pass)
            .expect("Rendering failed");
    }
}
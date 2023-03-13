mod texture;
mod camera;
mod transform;
mod input;
mod model;
mod resources;
mod device;
mod shaders;
mod scene;
mod frame_context;
mod render_target;
mod post_processor;
mod physics_world;
mod math;
mod imgui_winit;

use std::collections::VecDeque;
use std::time::Duration;
use imgui::{Condition, FontSource};
use imgui_wgpu::{Renderer, RendererConfig};
use winit::{event::*, event_loop::{ControlFlow, EventLoop}, window::WindowBuilder};
use winit::platform::run_return::EventLoopExtRunReturn;
use winit::window::CursorGrabMode;

use input::Input;
use device::Device;
use crate::device::SurfaceSize;
use crate::frame_context::FrameContext;
use crate::post_processor::PostProcessor;
use crate::scene::Scene;

async fn run() {
    let mut event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Demo")
        .with_inner_size(SurfaceSize::new(1800, 1200))
        .build(&event_loop)
        .unwrap();

    let mut device = Device::new(&window).await;
    let mut input = Input::new();

    let mut scene = Scene::new(&device).await;
    let mut post_processor = PostProcessor::new(&device, (200, 150)).await;

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

    imgui.fonts().add_font(&[FontSource::DefaultFontData {
        config: Some(imgui::FontConfig {
            oversample_h: 1,
            pixel_snap_h: true,
            size_pixels: font_size,
            ..Default::default()
        }),
    }]);

    let renderer_config = RendererConfig {
        texture_format: device.surface_texture_format(),
        ..Default::default()
    };

    let mut renderer = Renderer::new(&mut imgui, device.device(), device.queue(), renderer_config);

    const DT_FILTER_WIDTH: usize = 10;
    let mut dt_queue: VecDeque<f32> = VecDeque::with_capacity(DT_FILTER_WIDTH);
    let mut last_frame_instant = std::time::Instant::now();

    let mut last_cursor = None;

    let mut demo_ui_window_open = true;

    let mut running = true;
    while running {
        input.reset();

        event_loop.run_return(|event, _, flow| {
            *flow = ControlFlow::Poll;

            match event {
                Event::MainEventsCleared => {
                    *flow = ControlFlow::Exit;
                }

                Event::DeviceEvent {
                    event: DeviceEvent::MouseMotion { delta, },
                    ..
                } => {
                    input.on_mouse_move((delta.0 as f32, delta.1 as f32));
                },

                Event::WindowEvent {
                    ref event,
                    window_id,
                } if window_id == window.id() => {
                    match event {
                        WindowEvent::MouseInput {
                            state,
                            button,
                            ..
                        } => {
                            input.on_mouse_button(button, state);
                        }

                        WindowEvent::KeyboardInput {
                            input: KeyboardInput {
                                state: key_state,
                                virtual_keycode: Some(keycode),
                                ..
                            },
                            ..
                        } => {
                            input.on_key(keycode, key_state);
                        }

                        WindowEvent::Resized(new_size) => {
                            device.resize(*new_size);
                        },

                        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                            device.resize(**new_inner_size);
                        }

                        _ => ()
                    }
                }

                _ => {}
            }

            platform.handle_event(imgui.io_mut(), &window, &event);
        });

        if input.escape_down {
            running = false;
        }

        if input.rmb_down_just_switched {
            if input.rmb_down {
                window.set_cursor_grab(CursorGrabMode::Confined)
                    .or_else(|_e| window.set_cursor_grab(CursorGrabMode::Locked))
                    .unwrap();
                window.set_cursor_visible(false);
            } else {
                window.set_cursor_grab(CursorGrabMode::None).unwrap();
                window.set_cursor_visible(true);
            }
        }

        // Stolen from Kajiya
        let dt_filtered = {
            let now = std::time::Instant::now();
            let dt_duration = now - last_frame_instant;
            last_frame_instant = now;

            let dt_raw = dt_duration.as_secs_f32();

            if dt_queue.len() >= DT_FILTER_WIDTH {
                dt_queue.pop_front();
            }

            dt_queue.push_back(dt_raw);
            dt_queue.iter().copied().sum::<f32>() / dt_queue.len() as f32
        };

        let frame_context = FrameContext {
            dt: dt_filtered,
            input: &input,
        };

        imgui.io_mut().update_delta_time(Duration::from_secs_f32(dt_filtered));

        platform
            .prepare_frame(imgui.io_mut(), &window)
            .expect("Failed to prepare frame");
        let ui = imgui.frame();

        {
            let window = ui.window("Hello world");
            window
                .size([300.0, 100.0], Condition::FirstUseEver)
                .build(|| {
                    ui.text("Hello world!");
                    ui.text("This...is...imgui-rs on WGPU!");
                    ui.separator();
                    let mouse_pos = ui.io().mouse_pos;
                    ui.text(format!(
                        "Mouse Position: ({:.1},{:.1})",
                        mouse_pos[0], mouse_pos[1]
                    ));
                });

            let window = ui.window("Hello too");
            window
                .size([400.0, 200.0], Condition::FirstUseEver)
                .position([400.0, 200.0], Condition::FirstUseEver)
                .build(|| {
                    ui.text(format!("Frametime: {dt_filtered:?}"));
                });

            ui.show_demo_window(&mut demo_ui_window_open);
        }

        if last_cursor != Some(ui.mouse_cursor()) {
            last_cursor = Some(ui.mouse_cursor());
            platform.prepare_render(ui, &window);
        }

        let draw_data = imgui.render();
        device.render_ui(&mut renderer, draw_data);

        // scene.update(&frame_context);
        //
        // {
        //     let mut frame = device.new_frame(Some(post_processor.source_rt()));
        //     scene.render(&device, &mut frame);
        //     frame.finish(&device);
        // }
        //
        // {
        //     let mut frame = device.new_frame(None);
        //     post_processor.render(&mut frame);
        //     frame.finish(&device);
        // }
    }
}

fn main() {
    pollster::block_on(run());
}

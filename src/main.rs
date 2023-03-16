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
mod debug_ui;

use std::collections::VecDeque;
use winit::{event::*, event_loop::{ControlFlow, EventLoop}, window::WindowBuilder};
use winit::platform::run_return::EventLoopExtRunReturn;
use winit::window::CursorGrabMode;

use input::Input;
use device::Device;
use debug_ui::DebugUI;
use device::SurfaceSize;
use frame_context::FrameContext;
use post_processor::PostProcessor;
use scene::Scene;

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
    let mut pp = PostProcessor::new(&device, (200, 150)).await;

    let mut debug_ui = DebugUI::new(&device, &window);

    const DT_FILTER_WIDTH: usize = 10;
    let mut dt_queue: VecDeque<f32> = VecDeque::with_capacity(DT_FILTER_WIDTH);
    let mut last_frame_instant = std::time::Instant::now();

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

            debug_ui.handle_window_event(&window, &event);
        });

        if input.escape_down {
            running = false;
        }

        // Grab/release cursor
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
        let dt = {
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
            dt,
            input: &input,
            device: &device,
            window: &window
        };

        scene.update(&frame_context);
        debug_ui.update(&frame_context);

        {
            let mut frame = device.new_frame(Some(pp.source_rt()));
            scene.render(&mut frame, &frame_context);
            frame.render(None, &frame_context);
        }

        {
            let mut frame = device.new_frame(None);
            pp.render(&mut frame);
            frame.render(Some(&mut debug_ui), &frame_context);
        }
    }
}

fn main() {
    pollster::block_on(run());
}

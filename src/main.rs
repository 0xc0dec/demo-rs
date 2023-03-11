mod texture;
mod camera;
mod transform;
mod events;
mod model;
mod resources;
mod device;
mod shaders;
mod physics;
mod scene;
mod frame_context;
mod render_target;
mod post_processor;

use std::collections::VecDeque;
use winit::{event::*, event_loop::{ControlFlow, EventLoop}, window::WindowBuilder};
use winit::platform::run_return::EventLoopExtRunReturn;

use events::Events;
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
    let mut events = Events::new();

    let mut scene = Scene::new(&device).await;
    let mut post_processor = PostProcessor::new(&device, (200, 150)).await;

    const DT_FILTER_WIDTH: usize = 10;
    let mut dt_queue: VecDeque<f32> = VecDeque::with_capacity(DT_FILTER_WIDTH);
    let mut last_frame_instant = std::time::Instant::now();

    let mut running = true;
    while running {
        events.reset();

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
                    events.on_mouse_move((delta.0 as f32, delta.1 as f32));
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
                            events.on_mouse_button(button, state);
                        }

                        WindowEvent::KeyboardInput {
                            input: KeyboardInput {
                                state: key_state,
                                virtual_keycode: Some(keycode),
                                ..
                            },
                            ..
                        } => {
                            events.on_key(keycode, key_state);
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
        });

        if events.escape_down {
            running = false;
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
            events: &events,
        };

        scene.update(&frame_context);

        {
            let mut frame = device.new_frame(Some(post_processor.source_rt()));
            scene.render(&device, &mut frame);
            frame.finish(&device);
        }

        {
            let mut frame = device.new_frame(None);
            post_processor.render(&mut frame);
            frame.finish(&device);
        }
    }
}

fn main() {
    pollster::block_on(run());
}

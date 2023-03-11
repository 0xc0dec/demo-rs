mod texture;
mod camera;
mod transform;
mod events;
mod model;
mod resources;
mod device;
mod shaders;
mod state;
mod physics;
mod scene;
mod frame_context;
mod render_target;

use std::collections::VecDeque;
use wgpu::RenderBundleDescriptor;
use winit::{event::*, event_loop::{ControlFlow, EventLoop}, window::WindowBuilder};
use winit::platform::run_return::EventLoopExtRunReturn;

use events::Events;
use device::Device;
use crate::device::SurfaceSize;
use crate::frame_context::FrameContext;
use crate::shaders::{PostProcessShader, PostProcessShaderParams, Shader};
use crate::model::{DrawModel, Mesh};
use crate::render_target::RenderTarget;
use crate::state::State;

async fn run() {
    let mut event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Demo")
        .with_inner_size(SurfaceSize::new(1800, 1200))
        .build(&event_loop)
        .unwrap();

    let mut device = Device::new(&window).await;
    let mut events = Events::new();
    let mut state = State::new(&device).await;

    let rt = RenderTarget::new(&device, Some(SurfaceSize::new(200, 150)));
    let mut post_process_shader = PostProcessShader::new(&device, PostProcessShaderParams {
        texture: rt.color_tex()
    }).await;
    let post_process_quad = Mesh::quad(&device);

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

        state.update(&frame_context);

        device.render_to_target(&rt, {
            let mut encoder = device.new_render_encoder(Some(&rt));
            state.render(&device, &mut encoder);
            encoder.finish(&RenderBundleDescriptor { label: None })
        });

        device.render_to_surface({
            let mut encoder = device.new_render_encoder(None);
            post_process_shader.apply(&mut encoder);
            encoder.draw_mesh(&post_process_quad);
            encoder.finish(&RenderBundleDescriptor { label: None })
        });
    }
}

fn main() {
    pollster::block_on(run());
}

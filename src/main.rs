mod texture;
mod camera;
mod transform;
mod input;
mod model;
mod resources;
mod graphics;
mod render_target;
mod materials;
mod scene;
mod physics;

use std::collections::VecDeque;
use winit::{event::*, event_loop::{ControlFlow, EventLoop}, window::WindowBuilder};
use winit::dpi::{PhysicalSize, Size};
use winit::platform::run_return::EventLoopExtRunReturn;

use input::Input;
use graphics::Graphics;
use render_target::RenderTarget;
use crate::scene::Scene;

const DT_FILTER_WIDTH: usize = 10;

async fn run() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Demo")
        .with_inner_size(PhysicalSize::new(1600, 1200))
        .build(&event_loop)
        .unwrap();

    let mut dt_queue: VecDeque<f32> = VecDeque::with_capacity(DT_FILTER_WIDTH);
    let mut fake_dt_countdown: i32 = 1;
    let mut last_frame_instant = std::time::Instant::now();

    let mut gfx = Graphics::new(&window).await;
    let mut render_target = RenderTarget::new(&gfx, wgpu::Color {
        r: 0.0,
        g: 0.5,
        b: 0.0,
        a: 1.0,
    });
    let mut scene = Scene::new(&gfx).await;
    let mut input = Input::new();

    event_loop.run(move |event, _, flow| {
        *flow = ControlFlow::Poll;

        input.process_event(&event, &window.id());

        match event {
            Event::NewEvents(_) => {
                input.clear();
            }

            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => {
                match event {
                    WindowEvent::Resized(new_size) => {
                        gfx.resize(Some(*new_size));
                        scene.on_canvas_resize(*new_size);
                        render_target.resize(&gfx);
                    },
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        gfx.resize(Some(**new_inner_size));
                        scene.on_canvas_resize(**new_inner_size);
                        render_target.resize(&gfx);
                    }
                    _ => {}
                }
            }

            Event::MainEventsCleared => {
                // let dt = instant::Instant::now() - time;
                // time = instant::Instant::now();

                // Stolen from Kajiya
                let dt_filtered = {
                    let now = std::time::Instant::now();
                    let dt_duration = now - last_frame_instant;
                    last_frame_instant = now;

                    let dt_raw = dt_duration.as_secs_f32();

                    // >= because rendering (and thus the spike) happens _after_ this.
                    if fake_dt_countdown >= 0 {
                        // First frame. Return the fake value.
                        fake_dt_countdown -= 1;
                        dt_raw.min(1.0 / 60.0)
                    } else {
                        // Not the first frame. Start averaging.

                        if dt_queue.len() >= DT_FILTER_WIDTH {
                            dt_queue.pop_front();
                        }

                        dt_queue.push_back(dt_raw);
                        dt_queue.iter().copied().sum::<f32>() / dt_queue.len() as f32
                    }
                };

                scene.update(&input, dt_filtered);

                gfx.render_frame(&mut scene, &render_target);

                // TODO Restore
                // match state.render(&renderer) {
                //     Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                //         renderer.resize(None);
                //         state.resize(&renderer);
                //     },
                //     Err(wgpu::SurfaceError::OutOfMemory) => *flow = ControlFlow::Exit,
                //     _ => {}
                // }
            }

            _ => {}
        }

        if input.escape_down {
            *flow = ControlFlow::Exit;
        }
    });
}

fn main() {
    pollster::block_on(run());
}

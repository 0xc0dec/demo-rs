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

async fn run() {
    let mut event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Demo")
        .with_inner_size(PhysicalSize::new(1600, 1200))
        .build(&event_loop)
        .unwrap();

    const DT_FILTER_WIDTH: usize = 10;
    let mut dt_queue: VecDeque<f32> = VecDeque::with_capacity(DT_FILTER_WIDTH);
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

    let mut running = true;
    while running {
        input.clear();

        event_loop.run_return(|event, _, flow| {
            *flow = ControlFlow::Poll;

            match event {
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
                    *flow = ControlFlow::Exit;
                }

                _ => {}
            }

            input.process_event(&event, &window.id());
        });

        if input.escape_down {
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

        scene.update(&input, dt_filtered);

        gfx.render_frame(&mut scene, &render_target);
    }
}

fn main() {
    pollster::block_on(run());
}

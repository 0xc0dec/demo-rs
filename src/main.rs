mod texture;
mod camera;
mod transform;
mod events;
mod model;
mod resources;
mod graphics;
mod shaders;
mod state;
mod physics;
mod scene;
mod frame_context;
mod render_target;

use std::collections::VecDeque;
use winit::{event::*, event_loop::{ControlFlow, EventLoop}, window::WindowBuilder};
use winit::dpi::{PhysicalSize};
use winit::platform::run_return::EventLoopExtRunReturn;

use events::Events;
use graphics::Graphics;
use crate::frame_context::FrameContext;
use crate::shaders::{PostProcessShader, PostProcessShaderParams};
use crate::model::Mesh;
use crate::render_target::RenderTarget;
use crate::state::State;
use crate::texture::Texture;

async fn run() {
    let mut event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Demo")
        .with_inner_size(PhysicalSize::new(1800, 1200))
        .build(&event_loop)
        .unwrap();

    let mut gfx = Graphics::new(&window).await;
    let mut events = Events::new(&window);
    let mut state = State::new(&gfx).await;

    let rt = RenderTarget::new(&gfx, Some(PhysicalSize::new(200, 150)));
    let mut post_process_shader = PostProcessShader::new(&gfx, PostProcessShaderParams {
        texture: rt.color_tex()
    }).await;
    let post_process_quad = Mesh::quad(&gfx);

    const DT_FILTER_WIDTH: usize = 10;
    let mut dt_queue: VecDeque<f32> = VecDeque::with_capacity(DT_FILTER_WIDTH);
    let mut last_frame_instant = std::time::Instant::now();

    let mut running = true;
    while running {
        events.clear();

        event_loop.run_return(|event, _, flow| {
            *flow = ControlFlow::Poll;

            events.process_event(&event, &window.id());

            match event {
                Event::MainEventsCleared => {
                    *flow = ControlFlow::Exit;
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

        let mut frame_context = FrameContext {
            dt: dt_filtered,
            events: &events,
        };

        state.update(&frame_context);

        gfx.render_to_target(&rt, &mut state, &mut frame_context);
        gfx.render_to_surface(&mut post_process_shader, &post_process_quad, &mut frame_context);
    }
}

fn main() {
    pollster::block_on(run());
}

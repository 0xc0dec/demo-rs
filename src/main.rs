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

    let mut gfx = Device::new(&window).await;
    let mut events = Events::new(&window);
    let mut state = State::new(&gfx).await;

    let rt = RenderTarget::new(&gfx, Some(SurfaceSize::new(200, 150)));
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

        let frame_context = FrameContext {
            dt: dt_filtered,
            events: &events,
        };

        state.update(&frame_context);

        if let Some(new_size) = events.new_surface_size {
            gfx.resize(new_size);
        }

        gfx.render_to_target(&rt, {
            let mut encoder = gfx.new_render_encoder(Some(&rt));
            state.render(&gfx, &mut encoder, &frame_context);
            encoder.finish(&RenderBundleDescriptor { label: None })
        });

        gfx.render_to_surface({
            let mut encoder = gfx.new_render_encoder(None);
            post_process_shader.apply(&mut encoder);
            encoder.draw_mesh(&post_process_quad);
            encoder.finish(&RenderBundleDescriptor { label: None })
        });
    }
}

fn main() {
    pollster::block_on(run());
}

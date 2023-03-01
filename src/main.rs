mod state;
mod texture;
mod camera;
mod transform;
mod input;
mod model;
mod resources;
mod renderer;
mod material;

use winit::{event::*, event_loop::{ControlFlow, EventLoop}, window::WindowBuilder};

use state::State;
use crate::input::Input;
use crate::renderer::Renderer;

async fn run() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Testy Test")
        .build(&event_loop).unwrap();

    let mut renderer = Renderer::new(&window).await;
    let mut state = State::new(&renderer).await;
    let mut input = Input::new();
    let mut time = instant::Instant::now();

    event_loop.run(move |event, _, flow| {
        *flow = ControlFlow::Poll;

        input.process_event(&event, &window.id());

        match event {
            // TODO Use NewEvents, see docs

            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => {
                match event {
                    WindowEvent::Resized(new_size) => {
                        renderer.resize(Some(*new_size));
                        state.resize(&renderer);
                    },
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        renderer.resize(Some(**new_inner_size));
                        state.resize(&renderer);
                    }
                    _ => {}
                }
            }

            Event::MainEventsCleared => {
                let dt = instant::Instant::now() - time;
                time = instant::Instant::now();

                state.update(&input, dt.as_secs_f32());
                input.clear();

                match state.render(&renderer) {
                    Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                        renderer.resize(None);
                        state.resize(&renderer);
                    },
                    Err(wgpu::SurfaceError::OutOfMemory) => *flow = ControlFlow::Exit,
                    _ => {}
                }
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

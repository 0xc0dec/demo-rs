mod state;
mod texture;
mod camera;
mod transform;
mod input;

use winit::{event::*, event_loop::{ControlFlow, EventLoop}, window::WindowBuilder};
use state::State;
use crate::input::Input;

async fn run() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Testy Test")
        .build(&event_loop).unwrap();

    let mut state = State::new(window).await;
    let mut input = Input::new();
    let mut time = instant::Instant::now();

    event_loop.run(move |event, _, flow| {
        *flow = ControlFlow::Poll;

        input.process_event(&event, &state.window().id());

        match event {
            // TODO Use MainEventsCleared, see docs
            // TODO Use NewEvents, see docs

            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == state.window().id() => {
                match event {
                    WindowEvent::Resized(new_size) => state.resize(Some(*new_size)),
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        state.resize(Some(**new_inner_size))
                    }
                    _ => {}
                }
            }

            Event::MainEventsCleared => {
                let dt = instant::Instant::now() - time;
                time = instant::Instant::now();

                state.update(&input, dt.as_secs_f32());
                input.clear();

                match state.render() {
                    Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => state.resize(None),
                    Err(wgpu::SurfaceError::OutOfMemory) => *flow = ControlFlow::Exit,
                    _ => {}
                }
            }

            // Event::RedrawRequested(window_id) if window_id == state.window().id() => {
            //     let dt = instant::Instant::now() - last_render_time;
            //     last_render_time = instant::Instant::now();
            //
            //     state.update(&input, dt.as_secs_f32());
            //
            //     match state.render() {
            //         Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => state.resize(None),
            //         Err(wgpu::SurfaceError::OutOfMemory) => *flow = ControlFlow::Exit,
            //         _ => {}
            //     }
            // }

            // Event::RedrawEventsCleared => {
            //     state.window().request_redraw();
            // }

            _ => {}
        }

        if input.escape_down {
            *flow = ControlFlow::Exit;
        }
    })
}

fn main() {
    pollster::block_on(run());
}

mod state;
mod texture;

use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use state::State;

async fn run() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Testy Test")
        .build(&event_loop).unwrap();
    let mut state = State::new(window).await;

    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == state.window().id() => match event {
            WindowEvent::CloseRequested | WindowEvent::KeyboardInput {
                input:
                KeyboardInput {
                    state: ElementState::Pressed,
                    virtual_keycode: Some(VirtualKeyCode::Escape),
                    ..
                },
                ..
            } => *control_flow = ControlFlow::Exit,

            WindowEvent::Resized(new_size) => state.resize(Some(*new_size)),

            WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                state.resize(Some(**new_inner_size))
            }

            _ => {}
        }

        Event::RedrawRequested(window_id) if window_id == state.window().id() => {
            match state.render() {
                Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => state.resize(None),
                Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                _ => {}
            }
        }

        Event::RedrawEventsCleared => {
            state.window().request_redraw();
        }

        _ => {}
    })
}

fn main() {
    pollster::block_on(run());
}

mod state;
mod texture;
mod camera;

use winit::{event::*, event_loop::{ControlFlow, EventLoop}, window::WindowBuilder};
use state::State;

async fn run() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Testy Test")
        .build(&event_loop).unwrap();
    let mut state = State::new(window).await;

    let mut last_render_time = instant::Instant::now();
    event_loop.run(move |event, _, flow| {
        *flow = ControlFlow::Poll;
        let time_delta = instant::Instant::now() - last_render_time;
        last_render_time = instant::Instant::now();

        match event {
            Event::DeviceEvent {
                event: DeviceEvent::MouseMotion { delta, },
                ..
            } => {
                state.camera_controller.process_mouse_movement(delta, time_delta.as_secs_f32());
            },

            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == state.window().id() => {
                match event {
                    WindowEvent::MouseInput {
                        state: btn_state,
                        button,
                        ..
                    } => {
                        let down = *btn_state == ElementState::Pressed;
                        state.camera_controller.process_mouse_button(*button, down);
                    }

                    WindowEvent::KeyboardInput {
                        input: KeyboardInput {
                            state: key_state,
                            virtual_keycode: Some(keycode),
                            ..
                        },
                        ..
                    } => {
                        state.camera_controller.process_keyboard(
                            *keycode,
                            *key_state == ElementState::Pressed
                        );

                        match keycode {
                            VirtualKeyCode::Escape => *flow = ControlFlow::Exit,
                            _ => ()
                        }
                    }

                    WindowEvent::Resized(new_size) => state.resize(Some(*new_size)),

                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        state.resize(Some(**new_inner_size))
                    }

                    _ => {}
                }
            }

            Event::RedrawRequested(window_id) if window_id == state.window().id() => {
                state.update();
                match state.render() {
                    Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => state.resize(None),
                    Err(wgpu::SurfaceError::OutOfMemory) => *flow = ControlFlow::Exit,
                    _ => {}
                }
            }

            Event::RedrawEventsCleared => {
                state.window().request_redraw();
            }

            _ => {}
        }
    })
}

fn main() {
    pollster::block_on(run());
}

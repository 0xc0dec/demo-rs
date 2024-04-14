use winit::dpi::PhysicalSize;
use winit::event::{DeviceEvent, Event, KeyboardInput, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::platform::run_return::EventLoopExtRunReturn;
use winit::window::WindowBuilder;

fn main() {
    let mut event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Demo")
        .with_inner_size(PhysicalSize {
            width: 1900,
            height: 1200,
        })
        .build(&event_loop)
        .unwrap();
    let mut run = true;

    loop {
        event_loop.run_return(|event, _, flow| {
            *flow = ControlFlow::Poll;

            match event {
                Event::MainEventsCleared => {
                    *flow = ControlFlow::Exit;
                }

                Event::DeviceEvent {
                    event: DeviceEvent::MouseMotion { delta: _delta },
                    ..
                } => {}

                Event::WindowEvent {
                    ref event,
                    window_id,
                } if window_id == window.id() => match event {
                    WindowEvent::CloseRequested => {
                        // TODO Proper solution
                        run = false;
                    }

                    WindowEvent::MouseInput {
                        state: _state,
                        button: _btn,
                        ..
                    } => {}

                    WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                state: _key_state,
                                virtual_keycode: Some(_keycode),
                                ..
                            },
                        ..
                    } => {}

                    WindowEvent::Resized(_new_size) => {}

                    WindowEvent::ScaleFactorChanged {
                        new_inner_size: _new_inner_size,
                        ..
                    } => {}

                    _ => (),
                },

                _ => {}
            }
        });

        if !run {
            break;
        }
    }
}

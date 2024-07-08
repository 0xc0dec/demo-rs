use crate::events::{KeyboardEvent, MouseEvent, ResizeEvent};
use winit::dpi::PhysicalSize;
use winit::event::{DeviceEvent, ElementState, Event, KeyboardInput, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::platform::run_return::EventLoopExtRunReturn;
use winit::window::WindowBuilder;

use crate::resources::*;

mod assets;
mod components;
mod debug_ui;
mod events;
mod math;
mod resources;
mod systems;

// TODO After removing Bevy, refactor file structure, remove the notion of components/resources/systems.

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
    let mut device = pollster::block_on(Device::new(&window));
    let mut physics = PhysicsWorld::new();
    let mut input = Input::new();
    let mut frame_time = FrameTime::new();

    // TODO More optimal, this is an MVP
    let mut mouse_events = Vec::new();
    let mut keyboard_events = Vec::new();
    let mut resize_events = Vec::new();

    while !input.action_active(InputAction::Escape) {
        event_loop.run_return(|event, _, flow| {
            *flow = ControlFlow::Poll;

            match event {
                Event::MainEventsCleared => {
                    *flow = ControlFlow::Exit;
                }

                Event::DeviceEvent {
                    event: DeviceEvent::MouseMotion { delta },
                    ..
                } => {
                    mouse_events.push(MouseEvent::Move {
                        dx: delta.0 as f32,
                        dy: delta.1 as f32,
                    });
                }

                Event::WindowEvent {
                    ref event,
                    window_id,
                } if window_id == window.id() => match event {
                    WindowEvent::MouseInput { state, button, .. } => {
                        mouse_events.push(MouseEvent::Button {
                            btn: *button,
                            pressed: *state == ElementState::Pressed,
                        });
                    }

                    WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                state: key_state,
                                virtual_keycode: Some(keycode),
                                ..
                            },
                        ..
                    } => {
                        keyboard_events.push(KeyboardEvent {
                            code: *keycode,
                            pressed: *key_state == ElementState::Pressed,
                        });
                    }

                    WindowEvent::Resized(new_size) => {
                        resize_events.push(ResizeEvent(*new_size));
                    }

                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        resize_events.push(ResizeEvent(**new_inner_size));
                    }

                    _ => (),
                },

                _ => {}
            }

            // debug_ui.handle_window_event(&window, &event);
        });

        if let Some(e) = resize_events.last() {
            device.resize(e.0);
        }

        input.update2(&mouse_events, &keyboard_events);
        frame_time.update();
        physics.update(frame_time.delta);

        mouse_events.clear();
        keyboard_events.clear();
        resize_events.clear();
    }
}

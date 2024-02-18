use bevy_ecs::prelude::*;
use winit::event::{DeviceEvent, ElementState, Event, KeyboardInput, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::platform::run_return::EventLoopExtRunReturn;
use winit::window::Window;

use crate::debug_ui::DebugUI;
use crate::resources::events::{KeyboardEvent, MouseEvent, WindowResizeEvent};

pub fn handle_system_events(
    window: NonSend<Window>,
    mut event_loop: NonSendMut<EventLoop<()>>,
    mut debug_ui: NonSendMut<DebugUI>,
    mut resize_events: EventWriter<WindowResizeEvent>,
    mut keyboard_events: EventWriter<KeyboardEvent>,
    mut mouse_events: EventWriter<MouseEvent>,
) {
    event_loop.run_return(|event, _, flow| {
        *flow = ControlFlow::Poll;

        match event {
            Event::MainEventsCleared => {
                *flow = ControlFlow::Exit;
            }

            Event::DeviceEvent {
                event: DeviceEvent::MouseMotion { delta },
                ..
            } => mouse_events.send(MouseEvent::Move(delta.0 as f32, delta.1 as f32)),

            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => match event {
                WindowEvent::MouseInput { state, button, .. } => {
                    mouse_events.send(MouseEvent::Button {
                        button: *button,
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
                    keyboard_events.send(KeyboardEvent {
                        code: *keycode,
                        pressed: *key_state == ElementState::Pressed,
                    });
                }

                WindowEvent::Resized(new_size) => {
                    resize_events.send(WindowResizeEvent {
                        new_size: *new_size,
                    });
                }

                WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                    resize_events.send(WindowResizeEvent {
                        new_size: **new_inner_size,
                    });
                }

                _ => (),
            },

            _ => {}
        }

        debug_ui.handle_window_event(&window, &event);
    });
}

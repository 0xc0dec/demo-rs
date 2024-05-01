use bevy_ecs::prelude::*;
use winit::event::{DeviceEvent, ElementState, Event, KeyboardInput, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::platform::run_return::EventLoopExtRunReturn;
use winit::window::Window;

use crate::debug_ui::DebugUI;
use crate::events::ResizeEvent;
use crate::resources::Input;

pub fn capture_events(
    window: NonSend<Window>,
    mut input: ResMut<Input>,
    mut event_loop: NonSendMut<EventLoop<()>>,
    mut debug_ui: NonSendMut<DebugUI>,
    mut resize_events: EventWriter<ResizeEvent>,
) {
    input.reset();

    event_loop.run_return(|event, _, flow| {
        *flow = ControlFlow::Poll;

        match event {
            Event::MainEventsCleared => {
                *flow = ControlFlow::Exit;
            }

            Event::DeviceEvent {
                event: DeviceEvent::MouseMotion { delta },
                ..
            } => input.on_mouse_move((delta.0 as f32, delta.1 as f32)),

            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => match event {
                WindowEvent::MouseInput { state, button, .. } => {
                    input.on_mouse_button(*button, *state == ElementState::Pressed)
                }

                WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            state: key_state,
                            virtual_keycode: Some(keycode),
                            ..
                        },
                    ..
                } => input.on_key(*keycode, *key_state == ElementState::Pressed),

                WindowEvent::Resized(new_size) => {
                    resize_events.send(ResizeEvent(*new_size));
                }

                WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                    resize_events.send(ResizeEvent(**new_inner_size));
                }

                _ => (),
            },

            _ => {}
        }

        debug_ui.handle_window_event(&window, &event);
    });
}

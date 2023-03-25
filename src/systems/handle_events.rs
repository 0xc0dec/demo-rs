use bevy_ecs::prelude::*;
use winit::event::{DeviceEvent, Event, KeyboardInput, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::platform::run_return::EventLoopExtRunReturn;
use winit::window::{Window};
use crate::debug_ui::DebugUI;
use crate::device::Device;
use crate::input::Input;

pub fn handle_events(
    window: NonSend<Window>,
    mut event_loop: NonSendMut<EventLoop<()>>,
    mut input: NonSendMut<Input>,
    mut device: NonSendMut<Device>,
    mut debug_ui: NonSendMut<DebugUI>
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
            } => {
                input.on_mouse_move((delta.0 as f32, delta.1 as f32));
            }

            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => match event {
                WindowEvent::MouseInput { state, button, .. } => {
                    input.on_mouse_button(button, state);
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
                    input.on_key(keycode, key_state);
                }

                WindowEvent::Resized(new_size) => {
                    // TODO Save new size into a resource, in some other system react to it
                    // (e.g. via event)
                    device.resize(*new_size);
                }

                WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                    // TODO Save new size into a resource, in some other system react to it
                    // (e.g. via event)
                    device.resize(**new_inner_size);
                }

                _ => (),
            },

            _ => {}
        }

        debug_ui.handle_window_event(&window, &event);
    });
}

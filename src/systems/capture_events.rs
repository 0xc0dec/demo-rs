use bevy_ecs::prelude::*;
use winit::event::{DeviceEvent, ElementState, Event, KeyboardInput, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::platform::run_return::EventLoopExtRunReturn;
use winit::window::Window;

use crate::debug_ui::DebugUI;
use crate::resources::Events;

pub fn capture_events(
    window: NonSend<Window>,
    mut events: ResMut<Events>,
    mut event_loop: NonSendMut<EventLoop<()>>,
    mut debug_ui: NonSendMut<DebugUI>,
) {
    events.reset();

    event_loop.run_return(|event, _, flow| {
        *flow = ControlFlow::Poll;

        match event {
            Event::MainEventsCleared => {
                *flow = ControlFlow::Exit;
            }

            Event::DeviceEvent {
                event: DeviceEvent::MouseMotion { delta },
                ..
            } => events.on_mouse_move((delta.0 as f32, delta.1 as f32)),

            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => match event {
                WindowEvent::MouseInput { state, button, .. } => {
                    events.on_mouse_button(*button, *state == ElementState::Pressed)
                }

                WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            state: key_state,
                            virtual_keycode: Some(keycode),
                            ..
                        },
                    ..
                } => events.on_key(*keycode, *key_state == ElementState::Pressed),

                WindowEvent::Resized(new_size) => events.new_surface_size = Some(*new_size),

                WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                    events.new_surface_size = Some(**new_inner_size);
                }

                _ => (),
            },

            _ => {}
        }

        debug_ui.handle_window_event(&window, &event);
    });
}

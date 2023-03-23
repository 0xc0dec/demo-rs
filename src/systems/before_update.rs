use bevy_ecs::prelude::{NonSend, NonSendMut, ResMut};
use winit::event::{DeviceEvent, Event, KeyboardInput, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::platform::run_return::EventLoopExtRunReturn;
use winit::window::{CursorGrabMode, Window};
use crate::debug_ui::DebugUI;
use crate::device::Device;
use crate::input::Input;
use crate::state::State;

pub fn before_update(
    window: NonSend<Window>,
    mut state: ResMut<State>,
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
                    device.resize(*new_size);
                }

                WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                    device.resize(**new_inner_size);
                }

                _ => (),
            },

            _ => {}
        }

        debug_ui.handle_window_event(&window, &event);
    });

    if input.escape_down {
        state.running = false;
    }

    // Grab/release cursor
    if input.rmb_down_just_switched {
        if input.rmb_down {
            window
                .set_cursor_grab(CursorGrabMode::Confined)
                .or_else(|_e| window.set_cursor_grab(CursorGrabMode::Locked))
                .unwrap();
            window.set_cursor_visible(false);
        } else {
            window.set_cursor_grab(CursorGrabMode::None).unwrap();
            window.set_cursor_visible(true);
        }
    }

    state.frame_time.update();
}

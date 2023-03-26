mod init;
mod handle_system_events;
mod render;
mod build_debug_ui;

use bevy_ecs::prelude::{EventReader, NonSend, NonSendMut, Res, ResMut};
use winit::event::{MouseButton, VirtualKeyCode};
use winit::window::{CursorGrabMode, Window};
use crate::device::Device;
use crate::events::{KeyboardEvent, MouseEvent, WindowResizeEvent};
use crate::input_state::InputState;
use crate::physics_world::PhysicsWorld;
use crate::state::State;

pub use init::init;
pub use handle_system_events::handle_system_events;
pub use render::render;
pub use build_debug_ui::build_debug_ui;

pub fn resize_device(
    mut device: NonSendMut<Device>,
    mut events: EventReader<WindowResizeEvent>,
) {
    for evt in events.iter() {
        device.resize(evt.new_size);
    }
}

pub fn grab_cursor(
    window: NonSend<Window>,
    mut mouse_events: EventReader<MouseEvent>
) {
    for e in mouse_events.iter() {
        if let MouseEvent::Button { button, pressed } = e {
            if *button == MouseButton::Right {
                if *pressed {
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
        }
    }
}

pub fn escape_on_exit(
    mut state: ResMut<State>,
    mut keyboard_events: EventReader<KeyboardEvent>,
) {
    if keyboard_events
        .iter()
        .any(|e| e.code == VirtualKeyCode::Escape && e.pressed)
    {
        state.running = false;
    }
}

pub fn update_physics(mut physics: NonSendMut<PhysicsWorld>, state: Res<State>) {
    physics.update(state.frame_time.delta);
}

pub fn update_frame_time(mut state: ResMut<State>) {
    state.frame_time.update();
}

pub fn update_input_state(
    mut input: ResMut<InputState>,
    mut keyboard_events: EventReader<KeyboardEvent>,
    mut mouse_events: EventReader<MouseEvent>
) {
    input.reset();
    for e in keyboard_events.iter() {
        input.on_key(e.code, e.pressed);
    }

    for e in mouse_events.iter() {
        match e {
            MouseEvent::Move(dx, dy) => input.on_mouse_move((*dx, *dy)),
            MouseEvent::Button { button, pressed } => input.on_mouse_button(*button, *pressed)
        }
    }
}
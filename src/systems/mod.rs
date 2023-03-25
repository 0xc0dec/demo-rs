mod init;
mod handle_system_events;
mod render;
mod update_input_state;

use bevy_ecs::prelude::{EventReader, NonSend, NonSendMut, Res, ResMut};
use winit::event::VirtualKeyCode;
use winit::window::{CursorGrabMode, Window};
pub use init::init;
pub use handle_system_events::handle_system_events;
pub use render::render_frame;
use crate::device::Device;
use crate::events::{KeyboardEvent, WindowResized};
use crate::input::Input;
use crate::physics_world::PhysicsWorld;
use crate::state::State;

pub fn resize_device(
    mut device: NonSendMut<Device>,
    mut events: EventReader<WindowResized>,
) {
    for evt in events.iter() {
        device.resize(evt.new_size);
    }
}

pub fn grab_cursor(input: NonSend<Input>, window: NonSend<Window>) {
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
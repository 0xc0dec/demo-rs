use bevy_ecs::prelude::NonSend;
use winit::window::{CursorGrabMode, Window};
use crate::input::Input;

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
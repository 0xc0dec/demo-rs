use bevy_ecs::prelude::*;
use winit::event::MouseButton;
use winit::window::{CursorGrabMode, Window};

use crate::resources::events::MouseEvent;

pub fn grab_cursor(window: NonSend<Window>, mut mouse_events: EventReader<MouseEvent>) {
    for e in mouse_events.read() {
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

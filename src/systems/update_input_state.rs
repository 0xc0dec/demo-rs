use bevy_ecs::prelude::*;

use crate::resources::events::{KeyboardEvent, MouseEvent};
use crate::resources::Input;

pub fn update_input_state(
    mut input: ResMut<Input>,
    mut keyboard_events: EventReader<KeyboardEvent>,
    mut mouse_events: EventReader<MouseEvent>,
) {
    input.reset();
    for e in keyboard_events.read() {
        input.on_key(e.code, e.pressed);
    }

    for e in mouse_events.read() {
        match e {
            MouseEvent::Move(dx, dy) => input.on_mouse_move((*dx, *dy)),
            MouseEvent::Button { button, pressed } => input.on_mouse_button(*button, *pressed),
        }
    }
}

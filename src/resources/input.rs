use std::collections::HashMap;

use bevy_ecs::prelude::*;
use winit::event::*;

use crate::events::{KeyboardEvent, MouseEvent};

// TODO Refactor:
// - Instead of keys expose actions (e.g. Exit).
// - Make fields readonly (or expose a single func that would check an action by its ID).
#[derive(Resource)]
pub struct Input {
    pub lmb_down: bool,
    pub rmb_down: bool,
    pub mouse_delta: (f32, f32),

    key_pressed: HashMap<VirtualKeyCode, bool>,
    // Key presses from the previous frame
    key_prev_pressed: HashMap<VirtualKeyCode, bool>,
}

impl Input {
    pub fn new() -> Self {
        Input {
            lmb_down: false,
            rmb_down: false,
            mouse_delta: (0.0, 0.0),
            key_pressed: HashMap::new(),
            key_prev_pressed: HashMap::new(),
        }
    }

    pub fn key_pressed_first(&self, code: VirtualKeyCode) -> bool {
        let pressed = *self.key_pressed.get(&code).unwrap_or(&false);
        let last_pressed = *self.key_prev_pressed.get(&code).unwrap_or(&false);
        pressed && !last_pressed
    }

    pub fn key_pressed(&self, code: VirtualKeyCode) -> bool {
        *self.key_pressed.get(&code).unwrap_or(&false)
    }

    pub fn update(
        mut input: ResMut<Input>,
        mut mouse_events: EventReader<MouseEvent>,
        mut keyboard_events: EventReader<KeyboardEvent>,
    ) {
        input.mouse_delta = (0.0, 0.0);
        input.key_prev_pressed = input.key_pressed.clone();

        for e in mouse_events.read() {
            match *e {
                MouseEvent::Button { btn, pressed } => match btn {
                    MouseButton::Left => input.lmb_down = pressed,
                    MouseButton::Right => input.rmb_down = pressed,
                    _ => (),
                },

                MouseEvent::Move { dx, dy } => input.mouse_delta = (dx, dy),
            }
        }

        for &KeyboardEvent { code, pressed } in keyboard_events.read() {
            input.key_pressed.insert(code, pressed);
        }
    }
}

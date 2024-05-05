use std::collections::HashMap;

use bevy_ecs::prelude::*;
use winit::event::*;

use crate::events::{KeyboardEvent, MouseEvent};

pub enum InputAction {
    MoveForward,
    MoveBack,
    MoveLeft,
    MoveRight,
    MoveUp,
    MoveDown,
    Escape,
    ControlPlayer,
    Spawn,
    Grab,
}

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
enum Key {
    Keyboard(VirtualKeyCode),
    MouseButton(MouseButton),
}

// TODO Use something faster (non-heap) than hashmaps
#[derive(Resource)]
pub struct Input {
    // TODO Avoid public writeable field
    pub mouse_delta: (f32, f32),

    key_pressed: HashMap<Key, bool>,
    // Key presses from the previous frame
    key_prev_pressed: HashMap<Key, bool>,
}

impl Input {
    pub fn new() -> Self {
        Input {
            mouse_delta: (0.0, 0.0),
            key_pressed: HashMap::new(),
            key_prev_pressed: HashMap::new(),
        }
    }

    pub fn action_active(&self, action: InputAction) -> bool {
        self.key_pressed(action_key(action))
    }

    pub fn action_activated(&self, action: InputAction) -> bool {
        self.key_pressed_first(action_key(action))
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
                MouseEvent::Button { btn, pressed } => {
                    input.key_pressed.insert(Key::MouseButton(btn), pressed);
                }
                MouseEvent::Move { dx, dy } => {
                    input.mouse_delta = (dx, dy);
                }
            }
        }

        for &KeyboardEvent { code, pressed } in keyboard_events.read() {
            input.key_pressed.insert(Key::Keyboard(code), pressed);
        }
    }

    fn key_pressed_first(&self, key: Key) -> bool {
        let pressed = *self.key_pressed.get(&key).unwrap_or(&false);
        let last_pressed = *self.key_prev_pressed.get(&key).unwrap_or(&false);
        pressed && !last_pressed
    }

    fn key_pressed(&self, key: Key) -> bool {
        *self.key_pressed.get(&key).unwrap_or(&false)
    }
}

fn action_key(action: InputAction) -> Key {
    match action {
        InputAction::MoveForward => Key::Keyboard(VirtualKeyCode::W),
        InputAction::MoveBack => Key::Keyboard(VirtualKeyCode::S),
        InputAction::MoveLeft => Key::Keyboard(VirtualKeyCode::A),
        InputAction::MoveRight => Key::Keyboard(VirtualKeyCode::D),
        InputAction::MoveUp => Key::Keyboard(VirtualKeyCode::E),
        InputAction::MoveDown => Key::Keyboard(VirtualKeyCode::Q),
        InputAction::Escape => Key::Keyboard(VirtualKeyCode::Escape),
        InputAction::ControlPlayer => Key::Keyboard(VirtualKeyCode::Tab),
        InputAction::Spawn => Key::Keyboard(VirtualKeyCode::Space),
        InputAction::Grab => Key::MouseButton(MouseButton::Left),
    }
}

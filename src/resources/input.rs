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

#[derive(Resource)]
pub struct Input {
    mouse_delta: (f32, f32),
    // TODO Use something faster than hashmaps (non-heap)
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

    pub fn mouse_delta(&self) -> (f32, f32) {
        self.mouse_delta
    }

    pub fn action_active(&self, action: InputAction) -> bool {
        self.key_pressed(action_key(action))
    }

    pub fn action_activated(&self, action: InputAction) -> bool {
        self.key_pressed_first(action_key(action))
    }

    pub fn update(&mut self, mouse_events: &[MouseEvent], keyboard_events: &[KeyboardEvent]) {
        self.mouse_delta = (0.0, 0.0);
        self.key_prev_pressed = self.key_pressed.clone();

        for e in mouse_events {
            match *e {
                MouseEvent::Button { btn, pressed } => {
                    self.key_pressed.insert(Key::MouseButton(btn), pressed);
                }
                MouseEvent::Move { dx, dy } => {
                    self.mouse_delta = (dx, dy);
                }
            }
        }

        for &KeyboardEvent { code, pressed } in keyboard_events {
            self.key_pressed.insert(Key::Keyboard(code), pressed);
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

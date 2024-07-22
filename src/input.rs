use std::collections::HashMap;

use winit::event::*;
use winit::keyboard::KeyCode;

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
    Keyboard(KeyCode),
    MouseButton(MouseButton),
}

#[derive(Default)]
pub struct Input {
    mouse_delta: (f32, f32),
    // TODO Use something stack-based instead of HashMap?
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

    pub fn handle_keyboard_event(&mut self, code: KeyCode, pressed: bool) {
        self.key_pressed.insert(Key::Keyboard(code), pressed);
    }

    pub fn handle_mouse_button_event(&mut self, btn: MouseButton, pressed: bool) {
        self.key_pressed.insert(Key::MouseButton(btn), pressed);
    }

    pub fn handle_mouse_move_event(&mut self, dx: f32, dy: f32) {
        self.mouse_delta = (dx, dy);
    }

    pub fn clear(&mut self) {
        self.mouse_delta = (0.0, 0.0);
        self.key_prev_pressed = self.key_pressed.clone();
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
        InputAction::MoveForward => Key::Keyboard(KeyCode::KeyW),
        InputAction::MoveBack => Key::Keyboard(KeyCode::KeyS),
        InputAction::MoveLeft => Key::Keyboard(KeyCode::KeyA),
        InputAction::MoveRight => Key::Keyboard(KeyCode::KeyD),
        InputAction::MoveUp => Key::Keyboard(KeyCode::KeyE),
        InputAction::MoveDown => Key::Keyboard(KeyCode::KeyQ),
        InputAction::Escape => Key::Keyboard(KeyCode::Escape),
        InputAction::ControlPlayer => Key::Keyboard(KeyCode::Tab),
        InputAction::Spawn => Key::Keyboard(KeyCode::Space),
        InputAction::Grab => Key::MouseButton(MouseButton::Left),
    }
}

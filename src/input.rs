use std::collections::HashMap;

use winit::event::*;
use winit::keyboard::KeyCode;
use winit::keyboard::PhysicalKey::Code;

pub enum InputAction {
    MoveForward,
    MoveBack,
    MoveLeft,
    MoveRight,
    MoveUp,
    MoveDown,
    Quit,
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
    last_cursor_position: (f32, f32),
    cursor_in_window: bool,
    // TODO Use something stack-based instead of HashMap?
    key_pressed: HashMap<Key, bool>,
    // Key presses from the previous frame
    key_pressed_prev: HashMap<Key, bool>,
}

impl Input {
    pub fn new() -> Self {
        Input {
            mouse_delta: (0.0, 0.0),
            last_cursor_position: (0.0, 0.0),
            cursor_in_window: false,
            key_pressed: HashMap::new(),
            key_pressed_prev: HashMap::new(),
        }
    }

    pub fn mouse_delta(&self) -> (f32, f32) {
        self.mouse_delta
    }

    pub fn cursor_position(&self) -> Option<(f32, f32)> {
        if self.cursor_in_window {
            Some(self.last_cursor_position)
        } else {
            None
        }
    }

    pub fn action_active(&self, action: InputAction) -> bool {
        self.key_pressed(action_key(action))
    }

    pub fn action_activated(&self, action: InputAction) -> bool {
        self.key_pressed_first(action_key(action))
    }

    pub fn handle_device_event(&mut self, event: &DeviceEvent) {
        match event {
            DeviceEvent::MouseMotion { delta: (x, y) } => {
                self.mouse_delta.0 += *x as f32;
                self.mouse_delta.1 += *y as f32;
            }
            DeviceEvent::MouseWheel { .. } => (),
            _ => (),
        };
    }

    pub fn handle_window_event(&mut self, event: &WindowEvent) {
        match event {
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: Code(code),
                        state,
                        ..
                    },
                ..
            } => {
                self.key_pressed
                    .insert(Key::Keyboard(*code), *state == ElementState::Pressed);
            }

            WindowEvent::MouseInput { button, state, .. } => {
                self.key_pressed
                    .insert(Key::MouseButton(*button), *state == ElementState::Pressed);
            }

            WindowEvent::CursorEntered { .. } => {
                self.handle_cursor_entrance(true);
            }

            WindowEvent::CursorLeft { .. } => {
                self.handle_cursor_entrance(false);
            }

            WindowEvent::CursorMoved { position, .. } => {
                self.last_cursor_position = (position.x as f32, position.y as f32);
            }

            _ => (),
        };
    }

    pub fn clear(&mut self) {
        self.mouse_delta = (0.0, 0.0);
        self.key_pressed_prev.clone_from(&self.key_pressed);
    }

    fn handle_cursor_entrance(&mut self, entered: bool) {
        self.cursor_in_window = entered;
        // Reset all pressed mouse buttons when the cursor leaves
        if !entered {
            let keys = self.key_pressed.keys().cloned().collect::<Vec<_>>();
            for key in keys {
                if let Key::MouseButton(_) = key {
                    self.key_pressed.remove(&key);
                }
            }
        }
    }

    fn key_pressed_first(&self, key: Key) -> bool {
        let pressed = *self.key_pressed.get(&key).unwrap_or(&false);
        let last_pressed = *self.key_pressed_prev.get(&key).unwrap_or(&false);
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
        InputAction::Quit => Key::Keyboard(KeyCode::Escape),
        InputAction::ControlPlayer => Key::Keyboard(KeyCode::Tab),
        InputAction::Spawn => Key::Keyboard(KeyCode::KeyF),
        InputAction::Grab => Key::MouseButton(MouseButton::Left),
    }
}

use winit::event::MouseButton;
use winit::keyboard::KeyCode;

use crate::graphics::SurfaceSize;

pub struct ResizeEvent(pub SurfaceSize);

pub enum MouseEvent {
    Button { btn: MouseButton, pressed: bool },
    Move { dx: f32, dy: f32 },
}

pub struct KeyboardEvent {
    pub code: KeyCode,
    pub pressed: bool,
}

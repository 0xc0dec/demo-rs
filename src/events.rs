use crate::device::SurfaceSize;
use winit::event::{MouseButton, VirtualKeyCode};

pub struct ResizeEvent(pub SurfaceSize);

pub enum MouseEvent {
    Button { btn: MouseButton, pressed: bool },
    Move { dx: f32, dy: f32 },
}

pub struct KeyboardEvent {
    pub code: VirtualKeyCode,
    pub pressed: bool,
}

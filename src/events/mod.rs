use winit::event::{MouseButton, VirtualKeyCode};
use crate::device::SurfaceSize;

pub struct WindowResizeEvent {
    pub new_size: SurfaceSize
}

pub struct KeyboardEvent {
    pub code: VirtualKeyCode,
    pub pressed: bool
}

pub enum MouseEvent {
    Move(f32, f32),
    Button {
        button: MouseButton,
        pressed: bool
    }
}
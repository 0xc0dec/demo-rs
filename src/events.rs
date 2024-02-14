use bevy_ecs::prelude::Event;
use winit::event::{MouseButton, VirtualKeyCode};

use crate::device::SurfaceSize;

#[derive(Event)]
pub struct WindowResizeEvent {
    pub new_size: SurfaceSize,
}

#[derive(Event)]
pub struct KeyboardEvent {
    pub code: VirtualKeyCode,
    pub pressed: bool,
}

#[derive(Event)]
pub enum MouseEvent {
    Move(f32, f32),
    Button { button: MouseButton, pressed: bool },
}

use bevy_ecs::prelude::Event;
use winit::event::{MouseButton, VirtualKeyCode};

use crate::resources::SurfaceSize;

#[derive(Event)]
pub struct ResizeEvent(pub SurfaceSize);

#[derive(Event)]
pub enum MouseEvent {
    Button { btn: MouseButton, pressed: bool },
    Move { dx: f32, dy: f32 },
}

#[derive(Event)]
pub struct KeyboardEvent {
    pub code: VirtualKeyCode,
    pub pressed: bool,
}

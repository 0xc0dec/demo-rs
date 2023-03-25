use winit::event::VirtualKeyCode;
use crate::device::SurfaceSize;

// TODO Rename to WindowResizeEvent
pub struct WindowResized {
    pub new_size: SurfaceSize
}

pub struct KeyboardEvent {
    pub code: VirtualKeyCode,
    pub pressed: bool
}
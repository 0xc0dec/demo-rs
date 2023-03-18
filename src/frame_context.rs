use crate::device::Device;
use crate::input::Input;
use winit::window::Window;

pub struct FrameContext<'a> {
    pub dt: f32,
    pub input: &'a Input,
    pub device: &'a Device,
    pub window: &'a Window,
}

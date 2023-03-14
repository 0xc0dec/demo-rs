use winit::window::Window;
use crate::device::Device;
use crate::input::Input;

pub struct FrameContext<'a> {
    pub dt: f32,
    pub input: &'a Input,
    pub device: &'a Device,
    pub window: &'a Window
}
use crate::device::Device;
use crate::input::Input;
use crate::resources::Resources;

pub struct App {
    pub window: winit::window::Window,
    pub device: Device,
    pub resources: Resources,
    pub input: Input
}
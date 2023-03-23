use crate::device::Device;
use crate::input::Input;
use crate::assets::Assets;

pub struct App {
    pub window: winit::window::Window,
    pub device: Device,
    pub assets: Assets,
    pub input: Input
}
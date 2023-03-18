use crate::device::Device;
use crate::input::Input;
use crate::resources::Resources;

// TODO Rename to app?
pub struct State {
    pub device: Device,
    pub resources: Resources,
    pub input: Input
}
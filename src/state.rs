use crate::input::Input;
use crate::render::Renderer;
use std::sync::Arc;
use winit::window::Window;

// TODO Better name
pub struct State<'a> {
    pub window: Arc<Window>,
    pub renderer: Renderer<'a>,
    pub input: Input,
}

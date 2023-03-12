use crate::input::Input;

// TODO Rename to UpdateContext? Will it be used during rendering or only in update()?
pub struct FrameContext<'a> {
    pub dt: f32,
    pub input: &'a Input,
}
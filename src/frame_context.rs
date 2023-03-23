use crate::app::App;

// TODO Remove
pub struct FrameContext<'a> {
    pub dt: f32,
    pub app: &'a App,
}

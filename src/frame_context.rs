use crate::events::Events;
use crate::render_target::RenderTarget;

pub struct FrameContext<'a> {
    pub dt: f32,
    pub events: &'a Events,
    pub target: &'a mut RenderTarget
}
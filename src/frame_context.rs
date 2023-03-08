use crate::events::Events;
use crate::graphics::Graphics;

pub struct FrameContext<'a> {
    pub dt: f32,
    pub events: &'a Events,
}